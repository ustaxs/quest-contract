
use soroban_sdk::{contracttype, Address, Env, Map, String, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct YieldSource {
    pub id: u64,
    pub name: String,
    pub apy: u32, // Annual Percentage Yield in basis points
    pub risk_level: u32,
}

pub(crate) const STRATEGIES: &str = "STRATEGIES";

pub fn get_strategies(env: &Env) -> Map<Address, Vec<YieldSource>> {
    env.storage().instance().get(STRATEGIES).unwrap_or_default()
}

pub fn set_strategies(env: &Env, strategies: &Map<Address, Vec<YieldSource>>) {
    env.storage().instance().set(STRATEGIES, strategies);
}