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

    fn is_critical(&self, current_value: f32) -> bool {
        current_value > self.settings.critical_threshold
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

        // TODO Implement the logic
        Ok(CheckResult::new(self.name.clone(), CheckStatus::OK, None))
    }
}
