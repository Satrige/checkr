use super::super::{
    checker::{CheckResult, CheckStatus, Checker},
    ports::CpuSource,
};
use super::settings::CpuSettings;

pub struct CpuChecker<S: CpuSource> {
    settings: CpuSettings,
    name: String,
    source: S,
}

impl<S: CpuSource> CpuChecker<S> {
    pub fn new(settings: CpuSettings, source: S) -> Self {
        CpuChecker {
            settings,
            name: "cpu".to_string(),
            source,
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

impl<S: CpuSource> Checker for CpuChecker<S> {
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
            use crate::config::cpu_config::{CpuConfig, CpuThresholdsConfig};

            use super::*;

            #[test]
            fn it_should_fire_warning_because_of_one_minute_threshold() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_warning(&(1.1, 0.9, 0.9)), true);
            }

            #[test]
            fn it_should_fire_warning_because_of_five_minutes_threshold() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_warning(&(0.9, 1.1, 0.9)), true);
            }

            #[test]
            fn it_should_fire_warning_because_of_fifteen_minutes_threshold() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_warning(&(0.9, 0.9, 1.1)), true);
            }

            #[test]
            fn it_should_not_fire_warning() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_warning(&(0.9, 0.9, 0.9)), false);
            }
        }

        mod is_critical {
            use crate::config::cpu_config::{CpuConfig, CpuThresholdsConfig};

            use super::*;

            #[test]
            fn it_should_fire_critical_because_of_one_minute_threshold() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_critical(&(1.1, 0.9, 0.9)), true);
            }

            #[test]
            fn it_should_fire_critical_because_of_five_minutes_threshold() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_critical(&(0.9, 1.1, 0.9)), true);
            }

            #[test]
            fn it_should_fire_critical_because_of_fifteen_minutes_threshold() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_critical(&(0.9, 0.9, 1.1)), true);
            }

            #[test]
            fn it_should_not_fire_critical() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
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
                    .unwrap(),
                );

                assert_eq!(cpu_checker.is_critical(&(0.9, 0.9, 0.9)), false);
            }
        }
    }
}
