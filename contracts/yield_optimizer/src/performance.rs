use soroban_sdk::{contracttype, map, Env, Map, String, Symbol, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct PerformanceReport {
    pub asset: String,
    pub total_returns: u64,
    pub history: Vec<String>,
}

pub(crate) const PERFORMANCE: &str = "PERFORMANCE";

pub fn get_performance_report(env: &Env, asset: &String) -> Option<PerformanceReport> {
    let reports: Map<String, PerformanceReport> = env.storage().instance().get(PERFORMANCE).unwrap_or_else(|| map![env]);
    reports.get(asset.clone())
}

pub fn update_performance_report(env: &Env, asset: &String, new_return: u64, log_message: String) {
    let mut reports: Map<String, PerformanceReport> = env.storage().instance().get(PERFORMANCE).unwrap_or_else(|| map![env]);
    let mut report = reports.get(asset.clone()).unwrap_or(PerformanceReport {
        asset: asset.clone(),
        total_returns: 0,
        history: Vec::new(env),
    });

    report.total_returns += new_return;
    report.history.push_back(log_message);
    reports.set(asset.clone(), report);
    env.storage().instance().set(PERFORMANCE, &reports);
}