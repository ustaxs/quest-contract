#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

#[test]
fn test_add_and_get_strategies() {
    let env = Env::default();
    let contract_id = env.register_contract(None, YieldOptimizer);
    let client = YieldOptimizerClient::new(&env, &contract_id);

    let asset = Address::random(&env);
    let mut sources = Vec::new(&env);
    sources.push_back(YieldSource {
        id: 1,
        name: String::from_str(&env, "Source A"),
        apy: 500,
        risk_level: 1,
    });
    sources.push_back(YieldSource {
        id: 2,
        name: String::from_str(&env, "Source B"),
        apy: 600,
        risk_level: 2,
    });

    client.add_strategy(&asset, &sources);

    let strategies = client.get_strategies();
    assert_eq!(strategies.len(), 1);

    let sources = strategies.get(asset).unwrap();
    assert_eq!(sources.len(), 2);
}

#[test]
fn test_compare_yields() {
    let env = Env::default();
    let contract_id = env.register_contract(None, YieldOptimizer);
    let client = YieldOptimizerClient::new(&env, &contract_id);

    let asset = Address::random(&env);
    let mut sources = Vec::new(&env);
    sources.push_back(YieldSource {
        id: 1,
        name: String::from_str(&env, "Source A"),
        apy: 500,
        risk_level: 1,
    });
    sources.push_back(YieldSource {
        id: 2,
        name: String::from_str(&env, "Source B"),
        apy: 700,
        risk_level: 2,
    });
     sources.push_back(YieldSource {
        id: 3,
        name: String::from_str(&env, "Source C"),
        apy: 600,
        risk_level: 3,
    });

    client.add_strategy(&asset, &sources);

    let best_source = client.compare_yields(&asset).unwrap();
    assert_eq!(best_source.id, 2);
    assert_eq!(best_source.apy, 700);
}

#[test]
fn test_rebalance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, YieldOptimizer);
    let client = YieldOptimizerClient::new(&env, &contract_id);

    let asset = Address::random(&env);
    let mut sources = Vec::new(&env);
    sources.push_back(YieldSource {
        id: 1,
        name: String::from_str(&env, "Source A"),
        apy: 500,
        risk_level: 1,
    });
    sources.push_back(YieldSource {
        id: 2,
        name: String::from_str(&env, "Source B"),
        apy: 700,
        risk_level: 2,
    });
    client.add_strategy(&asset, &sources);

    client.rebalance(&asset);

    let report = client.get_performance_report(&asset).unwrap();
    assert_eq!(report.total_returns, 10);
}

#[test]
fn test_performance_report() {
    let env = Env::default();
    let contract_id = env.register_contract(None, YieldOptimizer);
    let client = YieldOptimizerClient::new(&env, &contract_id);

    let asset = Address::random(&env);
    let mut sources = Vec::new(&env);
    sources.push_back(YieldSource {
        id: 1,
        name: String::from_str(&env, "Source A"),
        apy: 500,
        risk_level: 2,
    });
    sources.push_back(YieldSource {
        id: 2,
        name: String::from_str(&env, "Source B"),
        apy: 700,
        risk_level: 5,
    });
    client.add_strategy(&asset, &sources);

    client.rebalance(&asset);

    let report = client.get_performance_report(&asset).unwrap();
    assert_eq!(report.total_returns, 10);
    assert_eq!(report.history.len(), 1);
}

#[test]
fn test_get_strategy_risk() {
    let env = Env::default();
    let contract_id = env.register_contract(None, YieldOptimizer);
    let client = YieldOptimizerClient::new(&env, &contract_id);

    let asset = Address::random(&env);
    let mut sources = Vec::new(&env);
    sources.push_back(YieldSource {
        id: 1,
        name: String::from_str(&env, "Source A"),
        apy: 500,
        risk_level: 2,
    });
    sources.push_back(YieldSource {
        id: 2,
        name: String::from_str(&env, "Source B"),
        apy: 700,
        risk_level: 5,
    });
    client.add_strategy(&asset, &sources);

    let total_risk = client.get_strategy_risk(&asset);
    assert_eq!(total_risk, 7);
}