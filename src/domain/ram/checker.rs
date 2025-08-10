use std::fs;

use super::super::checker::{CheckResult, CheckStatus, Checker};
use super::super::errors::CheckError;
use super::settings::RamSettings;

pub struct RamChecker {
    settings: RamSettings,
    name: String,
}

impl RamChecker {
    pub fn new(settings: RamSettings) -> Self {
        RamChecker {
            settings,
            name: "ram".to_string(),
        }
    }

    fn extract_kb_value(line: &str) -> f32 {
        line.split_whitespace()
            .nth(1)
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    fn calc_meminfo_usage(meminfo: &str) -> Result<f32, CheckError> {
        let mut mem_total = 0.0;
        let mut mem_available = 0.0;

        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total = Self::extract_kb_value(line);
            } else if line.starts_with("MemAvailable:") {
                mem_available = Self::extract_kb_value(line);
            }
        }

        if mem_total == 0.0 {
            return Err(CheckError::RamCheckError("MemTotal missing".into()));
        }

        Ok((mem_total - mem_available) * 100.0 / mem_total)
    }

    fn get_ram_usage_percent(&self) -> Result<f32, CheckError> {
        let meminfo = fs::read_to_string("/proc/meminfo")
            .map_err(|err| CheckError::RamCheckError(err.to_string()))?;

        Self::calc_meminfo_usage(&meminfo)
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

#[cfg(test)]
mod tests {
    use super::*;

    mod ram_checker {
        use super::*;

        mod extract_kb_value {
            use super::*;

            #[test]
            fn it_should_correctly_extract_kb_value() {
                assert_eq!(
                    RamChecker::extract_kb_value("MemTotal:        1921988 kB"),
                    1921988.0,
                );
            }

            #[test]
            fn it_should_fall_back_to_zero_value() {
                assert_eq!(
                    RamChecker::extract_kb_value("Just the random string without kb info"),
                    0.0,
                );
            }
        }

        mod calc_meminfo_usage {
            use super::*;

            #[test]
            fn it_should_correctly_calc_ram_usage_percent() {
                let meminfo = "\
MemTotal:       1000 kB
MemAvailable:    500 kB
                ";

                assert_eq!(RamChecker::calc_meminfo_usage(meminfo).unwrap(), 50.0,)
            }

            #[test]
            fn if_should_not_be_able_to_calc_ram_usage() {
                let meminfo = "Just the random string";

                let result = RamChecker::calc_meminfo_usage(meminfo);

                let err = result.unwrap_err();
                match err {
                    CheckError::RamCheckError(msg) => assert!(msg.contains("MemTotal")),
                    _ => panic!("Unexpected error type"),
                }
            }
        }

        mod is_warning {
            use crate::config::ram_config::RamConfig;

            use super::*;

            #[test]
            fn it_should_fire_warning_because_of_the_threshold() {
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                );

                assert_eq!(ram_checker.is_warning(90.0), true);
            }

            #[test]
            fn it_should_not_fire_warning() {
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                );

                assert_eq!(ram_checker.is_warning(79.0), false);
            }
        }

        mod is_critical {
            use crate::config::ram_config::RamConfig;

            use super::*;

            #[test]
            fn it_should_fire_critical_because_of_the_threshold() {
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                );

                assert_eq!(ram_checker.is_critical(91.0), true);
            }

            #[test]
            fn it_should_not_fire_critical() {
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                );

                assert_eq!(ram_checker.is_critical(89.0), false);
            }
        }
    }
}
