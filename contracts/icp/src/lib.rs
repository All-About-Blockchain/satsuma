use ic_cdk::api::caller;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

thread_local! {
    static BALANCES: RefCell<HashMap<String, u128>> = RefCell::new(HashMap::new());
}

#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
    pub principal: String,
    pub balance: u128,
}

#[update]
pub fn deposit_yield(amount: u128) {
    let caller_id = caller().to_text();
    BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        let entry = map.entry(caller_id).or_insert(0);
        *entry += amount;
    });
}

#[query]
pub fn get_balance(principal: String) -> BalanceResponse {
    BALANCES.with(|b| {
        let map = b.borrow();
        let bal = map.get(&principal).cloned().unwrap_or(0);
        BalanceResponse { principal, balance: bal }
    })
}

#[query]
pub fn my_balance() -> BalanceResponse {
    let caller_id = caller().to_text();
    BALANCES.with(|b| {
        let map = b.borrow();
        let bal = map.get(&caller_id).cloned().unwrap_or(0);
        BalanceResponse { principal: caller_id, balance: bal }
    })
} 