use crate::checkers::{
    Checker,
    cpu::{CpuChecker, CpuSettings, ProcLoadavg},
    disk_usage::{DiskUsageChecker, DiskUsageSettings, ProcDiskUsage},
    ram::{ProcMeminfo, RamChecker, RamSettings},
};
use crate::config::AppConfig;
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

    if let Some(disk_usage_config) = &config.disk_usage {
        let disk_usage_checker = DiskUsageChecker::new(
            DiskUsageSettings::try_from(disk_usage_config)?,
            ProcDiskUsage,
        );
        result.push(Arc::new(disk_usage_checker) as Arc<dyn Checker + Send + Sync>);
    }

    Ok(result)
}
