use ic_cdk::api::caller;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

thread_local! {
    static BALANCES: RefCell<HashMap<String, u128>> = RefCell::new(HashMap::new());
    static BITCOIN_BALANCES: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static INJECTIVE_CONFIG: RefCell<InjectiveConfig> = RefCell::new(InjectiveConfig::default());
    static YIELD_ACCUMULATOR: RefCell<u128> = RefCell::new(0);
    static TOTAL_BITCOIN_CONVERTED: RefCell<u64> = RefCell::new(0);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BalanceResponse {
    pub principal: String,
    pub balance: u128,
    pub bitcoin_balance: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitcoinBalanceResponse {
    pub principal: String,
    pub bitcoin_balance: u64,
    pub usd_value: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InjectiveConfig {
    pub contract_address: String,
    pub axelar_gateway: String,
    pub yield_collector: String,
    pub bitcoin_price_oracle: String,
}

impl Default for InjectiveConfig {
    fn default() -> Self {
        Self {
            contract_address: String::new(),
            axelar_gateway: String::new(),
            yield_collector: String::new(),
            bitcoin_price_oracle: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrossChainMessage {
    pub action: String,
    pub user: String,
    pub amount: u128,
    pub recipient: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitcoinConversionRequest {
    pub principal: String,
    pub usdc_amount: u128,
}

// Yield management functions
#[update]
pub fn deposit_yield(amount: u128) {
    let caller_id = caller().to_text();
    BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        let entry = map.entry(caller_id.clone()).or_insert(0);
        *entry += amount;
    });
    
    // Accumulate yield for Bitcoin conversion
    YIELD_ACCUMULATOR.with(|acc| {
        let mut total = acc.borrow_mut();
        *total += amount;
    });
    
    // Auto-convert to Bitcoin if threshold met
    convert_yield_to_bitcoin();
}

#[update]
pub fn convert_yield_to_bitcoin() {
    YIELD_ACCUMULATOR.with(|acc| {
        let mut total = acc.borrow_mut();
        if *total >= 100_000_000 { // 100 USDC minimum for conversion
            let conversion_amount = *total;
            *total = 0;
            
            // Convert to Bitcoin (simplified - in production would use price oracle)
            let bitcoin_amount = convert_usdc_to_bitcoin(conversion_amount);
            
            // Distribute Bitcoin to all users proportionally
            distribute_bitcoin_to_users(bitcoin_amount, conversion_amount);
            
            // Update total Bitcoin converted
            TOTAL_BITCOIN_CONVERTED.with(|total_btc| {
                let mut btc = total_btc.borrow_mut();
                *btc += bitcoin_amount;
            });
        }
    });
}

#[update]
pub fn manual_bitcoin_conversion(principal: String, usdc_amount: u128) {
    let caller_id = caller().to_text();
    if caller_id != principal {
        return; // Only allow self-conversion
    }
    
    // Check if user has enough yield balance
    let user_balance = BALANCES.with(|b| {
        let map = b.borrow();
        map.get(&principal).cloned().unwrap_or(0)
    });
    
    if user_balance < usdc_amount {
        return;
    }
    
    // Deduct from user's yield balance
    BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        if let Some(balance) = map.get_mut(&principal) {
            *balance -= usdc_amount;
        }
    });
    
    // Convert to Bitcoin
    let bitcoin_amount = convert_usdc_to_bitcoin(usdc_amount);
    
    // Add to user's Bitcoin balance
    BITCOIN_BALANCES.with(|btc| {
        let mut map = btc.borrow_mut();
        let entry = map.entry(principal).or_insert(0);
        *entry += bitcoin_amount;
    });
    
    // Update total Bitcoin converted
    TOTAL_BITCOIN_CONVERTED.with(|total_btc| {
        let mut btc = total_btc.borrow_mut();
        *btc += bitcoin_amount;
    });
}

// Injective management functions
#[update]
pub fn set_injective_config(config: InjectiveConfig) {
    let caller_id = caller().to_text();
    // Only allow authorized principals to set config
    if caller_id != "admin_principal_here" {
        return;
    }
    
    INJECTIVE_CONFIG.with(|cfg| {
        let mut config_ref = cfg.borrow_mut();
        *config_ref = config;
    });
}

#[update]
pub fn execute_injective_deposit(user: String, amount: u128) {
    let caller_id = caller().to_text();
    // Verify this is a cross-chain message from Injective
    if !is_valid_cross_chain_caller(&caller_id) {
        return;
    }
    
    // Update user's balance
    BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        let entry = map.entry(user.clone()).or_insert(0);
        *entry += amount;
    });
    
    // Trigger yield skimming on Injective
    trigger_injective_yield_skim(&user);
}

#[update]
pub fn trigger_injective_yield_skim(recipient: &str) {
    // This would send a cross-chain message to Injective to trigger yield skimming
    // In a real implementation, this would use Axelar GMP or similar
    let message = CrossChainMessage {
        action: "skim_yield".to_string(),
        user: "".to_string(),
        amount: 0,
        recipient: Some(recipient.to_string()),
    };
    
    // Send message to Injective (placeholder)
    send_cross_chain_message("injective", message);
}

// Query functions
#[query]
pub fn get_balance(principal: String) -> BalanceResponse {
    let balance = BALANCES.with(|b| {
        let map = b.borrow();
        map.get(&principal).cloned().unwrap_or(0)
    });
    
    let bitcoin_balance = BITCOIN_BALANCES.with(|btc| {
        let map = btc.borrow();
        map.get(&principal).cloned().unwrap_or(0)
    });
    
    BalanceResponse { 
        principal, 
        balance, 
        bitcoin_balance 
    }
}

#[query]
pub fn my_balance() -> BalanceResponse {
    let caller_id = caller().to_text();
    get_balance(caller_id)
}

#[query]
pub fn get_bitcoin_balance(principal: String) -> BitcoinBalanceResponse {
    let bitcoin_balance = BITCOIN_BALANCES.with(|btc| {
        let map = btc.borrow();
        map.get(&principal).cloned().unwrap_or(0)
    });
    
    // Calculate USD value (simplified - would use price oracle)
    let usd_value = bitcoin_balance as u128 * 45_000; // $45,000 per BTC
    
    BitcoinBalanceResponse {
        principal,
        bitcoin_balance,
        usd_value,
    }
}

#[query]
pub fn get_total_bitcoin_converted() -> u64 {
    TOTAL_BITCOIN_CONVERTED.with(|total| {
        *total.borrow()
    })
}

#[query]
pub fn get_yield_accumulator() -> u128 {
    YIELD_ACCUMULATOR.with(|acc| {
        *acc.borrow()
    })
}

#[query]
pub fn get_injective_config() -> InjectiveConfig {
    INJECTIVE_CONFIG.with(|cfg| {
        cfg.borrow().clone()
    })
}

// Helper functions
fn convert_usdc_to_bitcoin(usdc_amount: u128) -> u64 {
    // Simplified conversion - in production would use price oracle
    // Assuming 1 BTC = $45,000 and 6 decimal places for USDC
    let bitcoin_price_usd = 45_000_000_000; // $45,000 in microdollars
    let conversion = (usdc_amount * 1_000_000) / bitcoin_price_usd; // Convert to satoshis
    conversion as u64
}

fn distribute_bitcoin_to_users(bitcoin_amount: u64, total_yield: u128) {
    let user_balances = BALANCES.with(|b| {
        let map = b.borrow();
        map.clone()
    });
    
    for (principal, balance) in user_balances {
        if balance > 0 {
            let user_share = (balance as u64 * bitcoin_amount) / (total_yield as u64);
            if user_share > 0 {
                BITCOIN_BALANCES.with(|btc| {
                    let mut map = btc.borrow_mut();
                    let entry = map.entry(principal).or_insert(0);
                    *entry += user_share;
                });
            }
        }
    }
}

fn is_valid_cross_chain_caller(caller: &str) -> bool {
    // In production, this would verify the caller is from Axelar or authorized bridge
    caller.starts_with("axelar") || caller.starts_with("injective")
}

fn send_cross_chain_message(destination: &str, message: CrossChainMessage) {
    // Placeholder for cross-chain message sending
    // In production, this would use Axelar GMP or similar
    ic_cdk::api::print(format!("Sending message to {}: {:?}", destination, message));
}

// Admin functions
#[update]
pub fn set_bitcoin_price(price_usd: u128) {
    let caller_id = caller().to_text();
    // Only allow authorized principals
    if caller_id != "admin_principal_here" {
        return;
    }
    
    // Update Bitcoin price (would be stored in state)
    ic_cdk::api::print(format!("Bitcoin price updated to ${}", price_usd));
}

#[update]
pub fn emergency_withdraw(principal: String) {
    let caller_id = caller().to_text();
    if caller_id != principal {
        return;
    }
    
    // Return all balances to user
    let balance = BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        map.remove(&principal).unwrap_or(0)
    });
    
    let bitcoin_balance = BITCOIN_BALANCES.with(|btc| {
        let mut map = btc.borrow_mut();
        map.remove(&principal).unwrap_or(0)
    });
    
    ic_cdk::api::print(format!(
        "Emergency withdrawal for {}: {} USDC, {} BTC",
        principal, balance, bitcoin_balance
    ));
} 