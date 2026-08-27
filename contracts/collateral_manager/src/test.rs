#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_deposit_and_get_collateral() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);

    let collateral = client.get_collateral(&user, &asset);
    assert_eq!(collateral, 100);
}

#[test]
fn test_borrow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);
    client.borrow(&user, &asset, &50);

    let loan = client.get_loan(&user, &asset);
    assert_eq!(loan, 50);
}

#[test]
#[should_panic(expected = "Insufficient collateral")]
fn test_borrow_insufficient_collateral() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);
    client.borrow(&user, &asset, &150);
}

#[test]
fn test_withdraw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);
    client.borrow(&user, &asset, &50);
    client.withdraw(&user, &asset, &25);

    let collateral = client.get_collateral(&user, &asset);
    assert_eq!(collateral, 75);
}

#[test]
#[should_panic(expected = "Withdrawal would leave insufficient collateral")]
fn test_withdraw_insufficient_collateral() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);
    client.borrow(&user, &asset, &50);
    client.withdraw(&user, &asset, &75);
}

#[test]
fn test_repay() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);
    client.borrow(&user, &asset, &50);
    client.repay(&user, &asset, &25);

    let loan = client.get_loan(&user, &asset);
    assert_eq!(loan, 25);
}

#[test]
#[should_panic(expected = "Repayment amount exceeds loan balance")]
fn test_repay_exceeds_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.deposit(&user, &asset, &100);
    client.borrow(&user, &asset, &50);
    client.repay(&user, &asset, &75);
}

#[test]
fn test_interest_accrual() {
    let env = Env::default();
    env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let contract_id = env.register_contract(None, CollateralManager);
    let client = CollateralManagerClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let asset = Address::random(&env);

    client.set_interest_rate(&asset, &1000); // 10% APY, scaled by 100

    client.deposit(&user, &asset, &1000);
    client.borrow(&user, &asset, &100);

    env.ledger().with_mut(|li| {
        li.timestamp = 365 * 24 * 60 * 60; // One year
    });

    let loan = client.get_loan(&user, &asset);
    assert_eq!(loan, 110); // 100 (principal) + 10 (interest)
}