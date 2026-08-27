#![no_std]

mod performance;
mod storage;

use performance::{get_performance_report, update_performance_report, PerformanceReport};
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Vec};
use storage::{get_strategies, set_strategies, YieldSource};

#[contract]
pub struct YieldOptimizer;

#[contractimpl]
impl YieldOptimizer {
    pub fn add_strategy(env: Env, asset: Address, sources: Vec<YieldSource>) {
        let mut strategies = get_strategies(&env);
        strategies.set(asset, sources);
        set_strategies(&env, &strategies);
    }

    pub fn get_strategies(env: Env) -> Map<Address, Vec<YieldSource>> {
        get_strategies(&env)
    }

    pub fn compare_yields(env: Env, asset: Address) -> Option<YieldSource> {
        let strategies = get_strategies(&env);
        if let Some(sources) = strategies.get(asset) {
            sources.iter().max_by_key(|s| s.apy)
        } else {
            None
        }
    }

    pub fn rebalance(env: Env, asset: Address) {
        if let Some(best_source) = Self::compare_yields(env.clone(), asset.clone()) {
            let log_message = String::from_str(
                &env,
                &format!("Rebalanced to source {} with APY {}", best_source.id, best_source.apy),
            );
            update_performance_report(&env, &asset_to_string(asset), 10, log_message);
        }
    }

    pub fn get_performance_report(env: Env, asset: Address) -> Option<PerformanceReport> {
        get_performance_report(&env, &asset_to_string(asset))
    }

    pub fn get_strategy_risk(env: Env, asset: Address) -> u32 {
        let strategies = get_strategies(&env);
        if let Some(sources) = strategies.get(asset) {
            sources.iter().map(|s| s.risk_level).sum()
        } else {
            0
        }
    }

    pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

fn asset_to_string(asset: Address) -> String {
    String::from_str(&asset.env(), &format!("{}", asset))
}


#[cfg(test)]
mod test;