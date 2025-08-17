use crate::domain::DiskSnapshot;

use super::super::{
    checker::{CheckResult, CheckStatus, Checker},
    ports::DiskUsageSource,
};

use super::DiskUsageSettings;

pub struct DiskUsageChecker<S: DiskUsageSource> {
    settings: DiskUsageSettings,
    name: String,
    source: S,
}

impl<S: DiskUsageSource> DiskUsageChecker<S> {
    pub fn new(settings: DiskUsageSettings, source: S) -> Self {
        DiskUsageChecker {
            settings,
            name: "disk_usage".to_string(),
            source,
        }
    }

    fn is_warning(&self, current_value: f32) -> bool {
        current_value > self.settings.warning_threshold
    }

    fn get_warning_mounts(&self, disk_snapshots: &[DiskSnapshot]) -> Vec<DiskSnapshot> {
        disk_snapshots
            .iter()
            .filter(|disk_snapshot| self.is_warning(disk_snapshot.usage))
            .cloned()
            .collect()
    }

    fn is_critical(&self, current_value: f32) -> bool {
        current_value > self.settings.critical_threshold
    }

    fn get_critical_mounts(&self, disk_snapshots: &[DiskSnapshot]) -> Vec<DiskSnapshot> {
        disk_snapshots
            .iter()
            .filter(|disk_snapshot| self.is_critical(disk_snapshot.usage))
            .cloned()
            .collect()
    }

    fn compile_descr(disk_snapshots: &[DiskSnapshot]) -> String {
        disk_snapshots
            .iter()
            .map(|disk_snapshot| {
                format!(
                    "Disk usage for {} is {:.1}%",
                    disk_snapshot.mount, disk_snapshot.usage
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    }
}

impl<S: DiskUsageSource> Checker for DiskUsageChecker<S> {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn is_enabled(&self) -> bool {
        self.settings.enabled
    }

    fn check(&self) -> anyhow::Result<CheckResult> {
        if !self.is_enabled() {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::DISABLED,
                None,
            ));
        }

        let load_values = self.source.parse_values()?;

        let critical_mounts = self.get_critical_mounts(&load_values);
        if critical_mounts.len() > 0 {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::CRITICAL,
                Some(Self::compile_descr(&critical_mounts)),
            ));
        }

        let warning_mounts = self.get_warning_mounts(&load_values);
        if warning_mounts.len() > 0 {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::WARNING,
                Some(Self::compile_descr(&warning_mounts)),
            ));
        }

        Ok(CheckResult::new(self.name.clone(), CheckStatus::OK, None))
    }
}
