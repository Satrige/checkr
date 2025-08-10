use crate::{
    config::AppConfig, cpu::cpu_checker::CpuChecker, models::checker::Checker,
    ram::ram_checker::RamChecker,
};
use std::sync::Arc;

pub fn build_checkers(config: &AppConfig) -> anyhow::Result<Vec<Arc<dyn Checker + Send + Sync>>> {
    let mut result: Vec<Arc<dyn Checker + Send + Sync>> = Vec::new();

    if let Some(cpu_config) = &config.cpu {
        let cpu_checker = CpuChecker::new(cpu_config)?;
        result.push(Arc::new(cpu_checker) as Arc<dyn Checker + Send + Sync>);
    }

    if let Some(ram_config) = &config.ram {
        let ram_checker = RamChecker::new(ram_config)?;
        result.push(Arc::new(ram_checker) as Arc<dyn Checker + Send + Sync>);
    }

    Ok(result)
}
