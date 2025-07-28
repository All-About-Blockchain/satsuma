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
static ICP_MANAGER: Item<Addr> = Item::new("icp_manager");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub token_usdc: String,
    pub token_nusdc: String,
    pub astroport_router: String,
    pub axelar_gateway: String,
    pub icp_canister_id: String,
    pub yield_collector: Addr,
    pub pool_id_usdc_nusdc: u64,
    pub pool_id_nusdc_usdc: u64,
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
    pub pool_id_usdc_nusdc: u64,
    pub pool_id_nusdc_usdc: u64,
    pub icp_manager: String,
}

// Execute Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Deposit { amount: Uint128 },
    SkimYield {},
    SetIcpManager { manager: String },
    ExecuteFromIcp { action: IcpAction },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IcpAction {
    Deposit { user: String, amount: Uint128 },
    SkimYield { recipient: String },
    UpdateConfig { config: Config },
}

// Query Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Principal { address: String },
    TotalPrincipal {},
    NusdcBalance {},
    IcpManager {},
}

// CW20 helpers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cw20ExecuteMsg {
    pub transfer: Option<TransferMsg>,
    pub send: Option<SendMsg>,
    pub approve: Option<ApproveMsg>,
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
pub struct ApproveMsg {
    pub spender: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cw20QueryMsg {
    pub balance: Option<BalanceQuery>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BalanceQuery {
    pub address: String,
}

// Astroport Router Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AstroportSwapMsg {
    pub swap: AstroportSwap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AstroportSwap {
    pub offer_asset: OfferAsset,
    pub ask_asset_info: AssetInfo,
    pub minimum_receive: Option<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OfferAsset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    Token { contract_addr: String },
    NativeToken { denom: String },
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
    pub action: String,
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
        pool_id_usdc_nusdc: msg.pool_id_usdc_nusdc,
        pool_id_nusdc_usdc: msg.pool_id_nusdc_usdc,
    };
    CONFIG.save(deps.storage, &config)?;
    TOTAL_PRINCIPAL.save(deps.storage, &Uint128::zero())?;
    ICP_MANAGER.save(deps.storage, &deps.api.addr_validate(&msg.icp_manager)?)?;
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
        ExecuteMsg::SetIcpManager { manager } => execute_set_icp_manager(deps, info, manager),
        ExecuteMsg::ExecuteFromIcp { action } => execute_from_icp(deps, env, info, action),
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
        },
        QueryMsg::IcpManager {} => to_binary(&ICP_MANAGER.load(deps.storage)?),
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
    
    // 1. Transfer USDC from sender to contract
    let transfer_msg = Cw20ExecuteMsg {
        transfer: Some(TransferMsg {
            recipient: env.contract.address.to_string(),
            amount,
        }),
        send: None,
        approve: None,
    };
    
    // 2. Approve Astroport router to spend USDC
    let approve_msg = Cw20ExecuteMsg {
        transfer: None,
        send: None,
        approve: Some(ApproveMsg {
            spender: config.astroport_router.clone(),
            amount,
        }),
    };
    
    // 3. Swap USDC → nUSDC via Astroport router
    let swap_msg = build_astroport_swap_msg(&config, amount);
    
    // 4. Update principal
    PRINCIPAL_BALANCES.update(deps.storage, &info.sender, |val| {
        Ok(val.unwrap_or_default() + amount)
    })?;
    TOTAL_PRINCIPAL.update(deps.storage, |val| {
        Ok(val.unwrap_or_default() + amount)
    })?;
    
    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.token_usdc.clone(),
            msg: to_binary(&transfer_msg)?,
            funds: vec![],
        }))
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.token_usdc.clone(),
            msg: to_binary(&approve_msg)?,
            funds: vec![],
        }))
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
    // Only the yield_collector or ICP manager can call
    let icp_manager = ICP_MANAGER.load(deps.storage)?;
    if info.sender != config.yield_collector && info.sender != icp_manager {
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
    
    // Approve Astroport router to spend nUSDC
    let approve_msg = Cw20ExecuteMsg {
        transfer: None,
        send: None,
        approve: Some(ApproveMsg {
            spender: config.astroport_router.clone(),
            amount: yield_amt,
        }),
    };
    
    // Swap yield_amt nUSDC → USDC via Astroport
    let swap_msg = build_astroport_swap_msg_nusdc_to_usdc(&config, yield_amt);
    
    // Bridge to ICP via Axelar GMP
    let axelar_msg = build_axelar_gmp_msg(&config, yield_amt, config.yield_collector.to_string());
    
    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.token_nusdc.clone(),
            msg: to_binary(&approve_msg)?,
            funds: vec![],
        }))
        .add_message(swap_msg)
        .add_message(axelar_msg)
        .add_attribute("action", "skim_yield")
        .add_attribute("yield_amount", yield_amt))
}

// Set ICP manager (only current manager can call)
fn execute_set_icp_manager(
    deps: DepsMut,
    info: MessageInfo,
    manager: String,
) -> StdResult<Response> {
    let current_manager = ICP_MANAGER.load(deps.storage)?;
    if info.sender != current_manager {
        return Err(StdError::generic_err("Unauthorized"));
    }
    
    ICP_MANAGER.save(deps.storage, &deps.api.addr_validate(&manager)?)?;
    
    Ok(Response::new()
        .add_attribute("action", "set_icp_manager")
        .add_attribute("manager", manager))
}

// Execute actions from ICP
fn execute_from_icp(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: IcpAction,
) -> StdResult<Response> {
    let icp_manager = ICP_MANAGER.load(deps.storage)?;
    if info.sender != icp_manager {
        return Err(StdError::generic_err("Unauthorized"));
    }
    
    match action {
        IcpAction::Deposit { user, amount } => {
            let user_addr = deps.api.addr_validate(&user)?;
            PRINCIPAL_BALANCES.update(deps.storage, &user_addr, |val| {
                Ok(val.unwrap_or_default() + amount)
            })?;
            TOTAL_PRINCIPAL.update(deps.storage, |val| {
                Ok(val.unwrap_or_default() + amount)
            })?;
            
            Ok(Response::new()
                .add_attribute("action", "icp_deposit")
                .add_attribute("user", user)
                .add_attribute("amount", amount))
        },
        IcpAction::SkimYield { recipient } => {
            // Similar to execute_skim but with custom recipient
            let config = CONFIG.load(deps.storage)?;
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
            
            let approve_msg = Cw20ExecuteMsg {
                transfer: None,
                send: None,
                approve: Some(ApproveMsg {
                    spender: config.astroport_router.clone(),
                    amount: yield_amt,
                }),
            };
            
            let swap_msg = build_astroport_swap_msg_nusdc_to_usdc(&config, yield_amt);
            let axelar_msg = build_axelar_gmp_msg(&config, yield_amt, recipient);
            
            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: config.token_nusdc.clone(),
                    msg: to_binary(&approve_msg)?,
                    funds: vec![],
                }))
                .add_message(swap_msg)
                .add_message(axelar_msg)
                .add_attribute("action", "icp_skim_yield")
                .add_attribute("yield_amount", yield_amt))
        },
        IcpAction::UpdateConfig { config: new_config } => {
            CONFIG.save(deps.storage, &new_config)?;
            Ok(Response::new()
                .add_attribute("action", "update_config"))
        },
    }
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
fn build_astroport_swap_msg(config: &Config, amount: Uint128) -> CosmosMsg {
    let swap_msg = AstroportSwapMsg {
        swap: AstroportSwap {
            offer_asset: OfferAsset {
                info: AssetInfo::Token {
                    contract_addr: config.token_usdc.clone(),
                },
                amount,
            },
            ask_asset_info: AssetInfo::Token {
                contract_addr: config.token_nusdc.clone(),
            },
            minimum_receive: None,
        },
    };
    
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.astroport_router.clone(),
        msg: to_binary(&swap_msg).unwrap(),
        funds: vec![],
    })
}

// Helper: Build Astroport swap message (nUSDC → USDC)
fn build_astroport_swap_msg_nusdc_to_usdc(config: &Config, amount: Uint128) -> CosmosMsg {
    let swap_msg = AstroportSwapMsg {
        swap: AstroportSwap {
            offer_asset: OfferAsset {
                info: AssetInfo::Token {
                    contract_addr: config.token_nusdc.clone(),
                },
                amount,
            },
            ask_asset_info: AssetInfo::Token {
                contract_addr: config.token_usdc.clone(),
            },
            minimum_receive: None,
        },
    };
    
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.astroport_router.clone(),
        msg: to_binary(&swap_msg).unwrap(),
        funds: vec![],
    })
}

// Helper: Build Axelar GMP message
fn build_axelar_gmp_msg(config: &Config, amount: Uint128, recipient_principal: String) -> CosmosMsg {
    let payload = IcpPayload {
        principal: recipient_principal,
        amount,
        action: "deposit_yield".to_string(),
    };
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.axelar_gateway.clone(),
        msg: to_binary(&AxelarGmpMsg {
            destination_chain: "icp".to_string(),
            destination_address: config.icp_canister_id.clone(),
            payload: to_binary(&payload).unwrap(),
            symbol: "USDC".to_string(),
            amount,
        }).unwrap(),
        funds: vec![],
    })
} 