use soroban_sdk::{contracttype, Address, Env, Map, String};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Collateral {
    pub asset: Address,
    pub amount: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Loan {
    pub borrower: Address,
    pub amount: i128,
}

pub(crate) const COLLATERALS: &str = "COLLATERALS";
pub(crate) const LOANS: &str = "LOANS";
pub(crate) const INTEREST_RATES: &str = "INTEREST_RATES";
pub(crate) const LAST_UPDATE: &str = "LAST_UPDATE";

pub fn get_collaterals(env: &Env, user: &Address) -> Map<Address, i128> {
    let storage: Map<Address, Map<Address, i128>> = env.storage().instance().get(COLLATERALS).unwrap_or_default();
    storage.get(user.clone()).unwrap_or_default()
}

pub fn set_collaterals(env: &Env, user: &Address, collaterals: &Map<Address, i128>) {
    let mut storage: Map<Address, Map<Address, i128>> = env.storage().instance().get(COLLATERALS).unwrap_or_default();
    storage.set(user.clone(), collaterals.clone());
    env.storage().instance().set(COLLATERALS, &storage);
}

pub fn get_loans(env: &Env, user: &Address) -> Map<Address, i128> {
    let storage: Map<Address, Map<Address, i128>> = env.storage().instance().get(LOANS).unwrap_or_default();
    storage.get(user.clone()).unwrap_or_default()
}

pub fn set_loans(env: &Env, user: &Address, loans: &Map<Address, i128>) {
    let mut storage: Map<Address, Map<Address, i128>> = env.storage().instance().get(LOANS).unwrap_or_default();
    storage.set(user.clone(), loans.clone());
    env.storage().instance().set(LOANS, &storage);
}

pub fn get_interest_rate(env: &Env, asset: &Address) -> u32 {
    let rates: Map<Address, u32> = env.storage().instance().get(INTEREST_RATES).unwrap_or_default();
    rates.get(asset.clone()).unwrap_or(0)
}

pub fn set_interest_rate(env: &Env, asset: &Address, rate: u32) {
    let mut rates: Map<Address, u32> = env.storage().instance().get(INTEREST_RATES).unwrap_or_default();
    rates.set(asset.clone(), rate);
    env.storage().instance().set(INTEREST_RATES, &rates);
}

pub fn get_last_update_time(env: &Env, user: &Address, asset: &Address) -> u64 {
    let user_updates: Map<Address, Map<Address, u64>> = env.storage().instance().get(LAST_UPDATE).unwrap_or_default();
    let asset_updates = user_updates.get(user.clone()).unwrap_or_default();
    asset_updates.get(asset.clone()).unwrap_or(0)
}

pub fn set_last_update_time(env: &Env, user: &Address, asset: &Address, timestamp: u64) {
    let mut user_updates: Map<Address, Map<Address, u64>> = env.storage().instance().get(LAST_UPDATE).unwrap_or_default();
    let mut asset_updates = user_updates.get(user.clone()).unwrap_or_default();
    asset_updates.set(asset.clone(), timestamp);
    user_updates.set(user.clone(), asset_updates);
    env.storage().instance().set(LAST_UPDATE, &user_updates);
}