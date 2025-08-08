use std::fs;

use crate::cpu::cpu_config::CpuConfig;
use crate::cpu::cpu_settings::CpuSettings;
use crate::models::{
    checker::{CheckResult, CheckStatus, Checker},
    errors::CheckError,
};

pub struct CpuChecker {
    settings: CpuSettings,
    name: String,
}

impl CpuChecker {
    pub fn new(cpu_config: &CpuConfig) -> anyhow::Result<Self> {
        let settings = CpuSettings::try_from(cpu_config)?;

        Ok(CpuChecker {
            settings,
            name: "cpu".to_string(),
        })
    }

    fn get_cpu_usage(&self) -> Result<(f32, f32, f32), CheckError> {
        match fs::read_to_string("/proc/loadvg") {
            Ok(content) => {
                let parts: Vec<&str> = content.split_whitespace().collect();

                return Ok((
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                ));
            }
            Err(err) => Err(CheckError::CpuCheckError(err.to_string())),
        }
    }

    fn is_warning(&self, load_values: &(f32, f32, f32)) -> bool {
        load_values.0 > self.settings.warning.one_threshold
            || load_values.1 > self.settings.warning.five_threshold
            || load_values.2 > self.settings.warning.fifteen_threshold
    }

    fn is_critical(&self, load_values: &(f32, f32, f32)) -> bool {
        load_values.0 > self.settings.critical.one_threshold
            || load_values.1 > self.settings.critical.five_threshold
            || load_values.2 > self.settings.critical.fifteen_threshold
    }
}

impl Checker for CpuChecker {
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

        let load_values = self.get_cpu_usage()?;
        let (one, five, fifteen) = load_values;

        if self.is_critical(&load_values) {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::CRITICAL,
                Some(format!("one: {one}, five: {five}, fifteen: {fifteen}")),
            ));
        }

        if self.is_warning(&load_values) {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::WARNING,
                Some(format!("one: {one}, five: {five}, fifteen: {fifteen}")),
            ));
        }

        Ok(CheckResult::new(self.name.clone(), CheckStatus::OK, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cpu_checker {
        use super::*;

        mod is_warning {
            use crate::cpu::cpu_config::CpuThresholdsConfig;

            use super::*;

            #[test]
            fn it_should_fire_warning_because_of_one_minute_threshold() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_warning(&(1.1, 0.9, 0.9)), true);
            }

            #[test]
            fn it_should_fire_warning_because_of_five_minutes_threshold() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_warning(&(0.9, 1.1, 0.9)), true);
            }

            #[test]
            fn it_should_fire_warning_because_of_fifteen_minutes_threshold() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_warning(&(0.9, 0.9, 1.1)), true);
            }

            #[test]
            fn it_should_not_fire_warning() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_warning(&(0.9, 0.9, 0.9)), false);
            }
        }

        mod is_critical {
            use crate::cpu::cpu_config::CpuThresholdsConfig;

            use super::*;

            #[test]
            fn it_should_fire_critical_because_of_one_minute_threshold() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_critical(&(1.1, 0.9, 0.9)), true);
            }

            #[test]
            fn it_should_fire_critical_because_of_five_minutes_threshold() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_critical(&(0.9, 1.1, 0.9)), true);
            }

            #[test]
            fn it_should_fire_critical_because_of_fifteen_minutes_threshold() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_critical(&(0.9, 0.9, 1.1)), true);
            }

            #[test]
            fn it_should_not_fire_critical() {
                let cpu_checker = CpuChecker::new(&CpuConfig {
                    enabled: Some(true),
                    warning: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                    critical: Some(CpuThresholdsConfig {
                        one_threshold: 1.0,
                        five_threshold: 1.0,
                        fifteen_threshold: 1.0,
                    }),
                })
                .unwrap();

                assert_eq!(cpu_checker.is_critical(&(0.9, 0.9, 0.9)), false);
            }
        }
    }
}
