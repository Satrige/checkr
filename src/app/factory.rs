use crate::{
    config::AppConfig,
    domain::{
        checker::Checker,
        cpu::{checker::CpuChecker, settings::CpuSettings},
        ram::{checker::RamChecker, settings::RamSettings},
    },
    infra::{proc_loadavg::ProcLoadavg, proc_meminfo::ProcMeminfo},
};
use std::sync::Arc;

pub fn build_checkers(config: &AppConfig) -> anyhow::Result<Vec<Arc<dyn Checker + Send + Sync>>> {
    let mut result: Vec<Arc<dyn Checker + Send + Sync>> = Vec::new();

    if let Some(cpu_config) = &config.cpu {
        let cpu_checker = CpuChecker::new(CpuSettings::try_from(cpu_config)?, ProcLoadavg);
        result.push(Arc::new(cpu_checker) as Arc<dyn Checker + Send + Sync>);
    }

    if let Some(ram_config) = &config.ram {
        let ram_checker = RamChecker::new(RamSettings::try_from(ram_config)?, ProcMeminfo);
        result.push(Arc::new(ram_checker) as Arc<dyn Checker + Send + Sync>);
    }

    Ok(result)
}
