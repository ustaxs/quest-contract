#![no_std]

mod storage;

use soroban_sdk::{contract, contractimpl, Address, Env, Map, symbol_short};
use storage::{
    get_collaterals, set_collaterals, get_loans, set_loans, get_interest_rate, set_interest_rate,
    get_last_update_time, set_last_update_time,
};

#[contract]
pub struct CollateralManager;

#[contractimpl]
impl CollateralManager {
    pub fn set_interest_rate(env: Env, asset: Address, rate: u32) {
        set_interest_rate(&env, &asset, rate);
    }

    pub fn deposit(env: Env, user: Address, asset: Address, amount: i128) {
        let mut collaterals = get_collaterals(&env, &user);
        let current_balance = collaterals.get(asset.clone()).unwrap_or(0);
        collaterals.set(asset, current_balance + amount);
        set_collaterals(&env, &user, &collaterals);
    }

    pub fn get_collateral(env: Env, user: Address, asset: Address) -> i128 {
        get_collaterals(&env, &user).get(asset).unwrap_or(0)
    }

    pub fn borrow(env: Env, user: Address, asset: Address, amount: i128) {
        let collaterals = get_collaterals(&env, &user);
        let collateral_balance = collaterals.get(asset.clone()).unwrap_or(0);

        let mut loans = get_loans(&env, &user);
        let loan_balance = Self::get_loan_with_interest(&env, &user, &asset);

        if collateral_balance < loan_balance + amount {
            panic!("Insufficient collateral");
        }

        loans.set(asset.clone(), loan_balance + amount);
        set_loans(&env, &user, &loans);
        set_last_update_time(&env, &user, &asset, env.ledger().timestamp());
    }

    pub fn get_loan(env: Env, user: Address, asset: Address) -> i128 {
        Self::get_loan_with_interest(&env, &user, &asset)
    }

    pub fn withdraw(env: Env, user: Address, asset: Address, amount: i128) {
        let mut collaterals = get_collaterals(&env, &user);
        let collateral_balance = collaterals.get(asset.clone()).unwrap_or(0);

        let loan_balance = Self::get_loan_with_interest(&env, &user, &asset);

        if collateral_balance - amount < loan_balance {
            panic!("Withdrawal would leave insufficient collateral");
        }

        collaterals.set(asset, collateral_balance - amount);
        set_collaterals(&env, &user, &collaterals);
    }

    pub fn repay(env: Env, user: Address, asset: Address, amount: i128) {
        let mut loans = get_loans(&env, &user);
        let loan_balance = Self::get_loan_with_interest(&env, &user, &asset);

        if amount > loan_balance {
            panic!("Repayment amount exceeds loan balance");
        }

        loans.set(asset.clone(), loan_balance - amount);
        set_loans(&env, &user, &loans);
        set_last_update_time(&env, &user, &asset, env.ledger().timestamp());
    }

    fn get_loan_with_interest(env: &Env, user: &Address, asset: &Address) -> i128 {
        let loan_balance = get_loans(env, user).get(asset.clone()).unwrap_or(0);
        if loan_balance == 0 {
            return 0;
        }

        let last_update_time = get_last_update_time(env, user, asset);
        let current_time = env.ledger().timestamp();
        let interest_rate = get_interest_rate(env, asset);

        let time_diff = current_time - last_update_time;
        let interest = (loan_balance * (interest_rate as i128) * (time_diff as i128)) / (365 * 24 * 60 * 60 * 100); // APY

        loan_balance + interest
    }
}

#[cfg(test)]
mod test;