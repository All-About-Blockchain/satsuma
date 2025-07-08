// CosmWasm Vault Contract for Injective
// Yield-skimming nUSDC strategy
// Written for Injective Chain using Astroport + Axelar GMP

use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128, WasmMsg, Storage, BankMsg, Coin, SubMsg, QueryRequest, WasmQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::{Map, Item};
use serde::{Deserialize, Serialize};

// Constants
const CONTRACT_NAME: &str = "injective_yield_vault";
const CONTRACT_VERSION: &str = "1.0.0";

// Storage
static PRINCIPAL_BALANCES: Map<&Addr, Uint128> = Map::new("principal");
static TOTAL_PRINCIPAL: Item<Uint128> = Item::new("total_principal");
static CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub token_usdc: String,
    pub token_nusdc: String,
    pub astroport_router: String,
    pub axelar_gateway: String,
    pub icp_canister_id: String,
    pub yield_collector: Addr,
}

// Instantiate Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub token_usdc: String,
    pub token_nusdc: String,
    pub astroport_router: String,
    pub axelar_gateway: String,
    pub icp_canister_id: String,
    pub yield_collector: String,
}

// Execute Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Deposit { amount: Uint128 },
    SkimYield {},
}

// Query Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Principal { address: String },
    TotalPrincipal {},
    NusdcBalance {},
}

// CW20 helpers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cw20ExecuteMsg {
    pub transfer: Option<TransferMsg>,
    pub send: Option<SendMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransferMsg {
    pub recipient: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SendMsg {
    pub contract: String,
    pub amount: Uint128,
    pub msg: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cw20QueryMsg {
    pub balance: Option<BalanceQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BalanceQuery {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AxelarGmpMsg {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Binary,
    pub symbol: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IcpPayload {
    pub principal: String,
    pub amount: Uint128,
}

// Instantiate
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {
        token_usdc: msg.token_usdc,
        token_nusdc: msg.token_nusdc,
        astroport_router: msg.astroport_router,
        axelar_gateway: msg.axelar_gateway,
        icp_canister_id: msg.icp_canister_id,
        yield_collector: deps.api.addr_validate(&msg.yield_collector)?,
    };
    CONFIG.save(deps.storage, &config)?;
    TOTAL_PRINCIPAL.save(deps.storage, &Uint128::zero())?;
    Ok(Response::default())
}

// Execute
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Deposit { amount } => execute_deposit(deps, env, info, amount),
        ExecuteMsg::SkimYield {} => execute_skim(deps, env, info),
    }
}

// Query
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::Principal { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let bal = PRINCIPAL_BALANCES.may_load(deps.storage, &addr)?.unwrap_or_default();
            to_binary(&bal)
        },
        QueryMsg::TotalPrincipal {} => to_binary(&TOTAL_PRINCIPAL.load(deps.storage)?),
        QueryMsg::NusdcBalance {} => {
            let config = CONFIG.load(deps.storage)?;
            let bal = query_cw20_balance(
                deps,
                &config.token_nusdc,
                &env.contract.address.to_string(),
            )?;
            to_binary(&bal)
        }
    }
}

// Handle deposit: USDC → swap → nUSDC → update principal
fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    // 1. Transfer USDC from sender to contract (assume user has approved contract)
    // 2. Swap USDC → nUSDC via Astroport router
    let swap_msg = build_astroport_swap_msg(&config, &info.sender, amount);
    // 3. Update principal
    PRINCIPAL_BALANCES.update(deps.storage, &info.sender, |val| {
        Ok(val.unwrap_or_default() + amount)
    })?;
    TOTAL_PRINCIPAL.update(deps.storage, |val| {
        Ok(val.unwrap_or_default() + amount)
    })?;
    Ok(Response::new()
        .add_message(swap_msg)
        .add_attribute("action", "deposit")
        .add_attribute("amount", amount))
}

// Skim yield and bridge to ICP
fn execute_skim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    // Only the yield_collector can call
    if info.sender != config.yield_collector {
        return Err(StdError::generic_err("Unauthorized"));
    }
    // Get nUSDC balance of contract
    let current_balance = query_cw20_balance(
        deps.as_ref(),
        &config.token_nusdc,
        &env.contract.address.to_string(),
    )?;
    let total_principal = TOTAL_PRINCIPAL.load(deps.storage)?;
    if current_balance <= total_principal {
        return Err(StdError::generic_err("No yield available"));
    }
    let yield_amt = current_balance - total_principal;
    // Swap yield_amt nUSDC → USDC via Astroport
    let swap_msg = build_astroport_swap_msg_nusdc_to_usdc(&config, yield_amt);
    // Bridge to ICP via Axelar GMP, sending yield_collector principal as recipient
    let axelar_msg = build_axelar_gmp_msg(&config, yield_amt, config.yield_collector.to_string());
    Ok(Response::new()
        .add_message(swap_msg)
        .add_message(axelar_msg)
        .add_attribute("action", "skim_yield")
        .add_attribute("yield_amount", yield_amt))
}

// Helper: Query CW20 balance
fn query_cw20_balance(
    deps: Deps,
    token: &str,
    address: &str,
) -> StdResult<Uint128> {
    let msg = Cw20QueryMsg {
        balance: Some(BalanceQuery {
            address: address.to_string(),
        }),
    };
    let res: cw20::BalanceResponse = deps.querier.query(&QueryRequest::WasmSmart {
        contract_addr: token.to_string(),
        msg: to_binary(&msg)?,
    })?;
    Ok(res.balance)
}

// Helper: Build Astroport swap message (USDC → nUSDC)
fn build_astroport_swap_msg(config: &Config, sender: &Addr, amount: Uint128) -> CosmosMsg {
    // TODO: Fill in Astroport swap message details for USDC → nUSDC
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.astroport_router.clone(),
        msg: to_binary(&"astroport_swap_msg_placeholder") // Replace with actual swap msg
            .unwrap(),
        funds: vec![],
    })
}

// Helper: Build Astroport swap message (nUSDC → USDC)
fn build_astroport_swap_msg_nusdc_to_usdc(config: &Config, amount: Uint128) -> CosmosMsg {
    // TODO: Fill in Astroport swap message details for nUSDC → USDC
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.astroport_router.clone(),
        msg: to_binary(&"astroport_swap_msg_placeholder") // Replace with actual swap msg
            .unwrap(),
        funds: vec![],
    })
}

// Helper: Build Axelar GMP message
fn build_axelar_gmp_msg(config: &Config, amount: Uint128, recipient_principal: String) -> CosmosMsg {
    let payload = IcpPayload {
        principal: recipient_principal,
        amount,
    };
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.axelar_gateway.clone(),
        msg: to_binary(&AxelarGmpMsg {
            destination_chain: "icp".to_string(),
            destination_address: config.icp_canister_id.clone(),
            payload: to_binary(&payload).unwrap(),
            symbol: "nUSDC".to_string(),
            amount,
        }).unwrap(),
        funds: vec![],
    })
} 