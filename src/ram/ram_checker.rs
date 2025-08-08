use std::fs;

use crate::{
    models::{
        checker::{CheckResult, CheckStatus, Checker},
        errors::CheckError,
    },
    ram::{ram_config::RamConfig, ram_settings::RamSettings},
};

pub struct RamChecker {
    settings: RamSettings,
    name: String,
}

impl RamChecker {
    pub fn new(ram_config: &RamConfig) -> anyhow::Result<Self> {
        let settings = RamSettings::try_from(ram_config)?;

        Ok(RamChecker {
            settings,
            name: "ram".to_string(),
        })
    }

    fn extract_kb_value(line: &str) -> f32 {
        line.split_whitespace()
            .nth(1)
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    fn get_ram_usage_percent(&self) -> Result<f32, CheckError> {
        match fs::read_to_string("/proc/meminfo") {
            Ok(meminfo) => {
                let mut mem_total = 0.0;
                let mut mem_available = 0.0;

                for line in meminfo.lines() {
                    if line.starts_with("MemTotal:") {
                        mem_total = Self::extract_kb_value(line);
                    } else if line.starts_with("MemAvailable:") {
                        mem_available = Self::extract_kb_value(line);
                    }
                }

                return Ok((mem_total - mem_available) * 100.0 / mem_total);
            }
            Err(err) => Err(CheckError::RamCheckError(err.to_string())),
        }
    }

    fn is_warning(&self, current_value: f32) -> bool {
        current_value > self.settings.warning_threshold
    }

    fn is_critical(&self, current_value: f32) -> bool {
        current_value > self.settings.critical_threshold
    }
}

impl Checker for RamChecker {
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

        let current_value = self.get_ram_usage_percent()?;

        if self.is_critical(current_value) {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::CRITICAL,
                Some(format!("usage: {current_value}%")),
            ));
        }

        if self.is_warning(current_value) {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::WARNING,
                Some(format!("usage: {current_value}%")),
            ));
        }

        Ok(CheckResult::new(self.name.clone(), CheckStatus::OK, None))
    }
}
