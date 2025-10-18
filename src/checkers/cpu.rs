mod config;
mod proc_loadavg;
mod settings;

use super::{CheckResult, CheckStatus, Checker};
pub use config::*;
pub use proc_loadavg::ProcLoadavg;
pub use settings::CpuSettings;

pub trait CpuSource: Send + Sync {
    fn parse_values(&self) -> anyhow::Result<(f32, f32, f32)>;
}

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
    use proc_loadavg::CpuParseError;

    struct FakeCpuSource {
        one_value: f32,
        five_value: f32,
        fifteen_value: f32,
    }

    impl Default for FakeCpuSource {
        fn default() -> Self {
            FakeCpuSource {
                one_value: 0.0,
                five_value: 0.0,
                fifteen_value: 0.0,
            }
        }
    }

    impl CpuSource for FakeCpuSource {
        fn parse_values(&self) -> anyhow::Result<(f32, f32, f32)> {
            Ok((self.one_value, self.five_value, self.fifteen_value))
        }
    }

    mod cpu_checker {
        use super::*;

        mod is_warning {
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
                    FakeCpuSource::default(),
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
                    FakeCpuSource::default(),
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
                    FakeCpuSource::default(),
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
                    FakeCpuSource::default(),
                );

                assert_eq!(cpu_checker.is_warning(&(0.9, 0.9, 0.9)), false);
            }
        }

        mod is_critical {
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
                    FakeCpuSource::default(),
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
                    FakeCpuSource::default(),
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
                    FakeCpuSource::default(),
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
                    FakeCpuSource::default(),
                );

                assert_eq!(cpu_checker.is_critical(&(0.9, 0.9, 0.9)), false);
            }
        }

        mod check {
            use super::*;

            #[test]
            fn it_should_be_disabled() {
                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
                        enabled: Some(false),
                        warning: None,
                        critical: None,
                    })
                    .unwrap(),
                    FakeCpuSource::default(),
                );

                assert_eq!(
                    cpu_checker.check().unwrap(),
                    CheckResult::new(String::from("cpu"), CheckStatus::DISABLED, None),
                );
            }

            #[test]
            fn it_should_be_critical() {
                let one_value = 1.1;
                let five_value = 1.1;
                let fifteen_value = 1.1;

                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
                        enabled: Some(true),
                        warning: Some(CpuThresholdsConfig {
                            one_threshold: 0.8,
                            five_threshold: 0.8,
                            fifteen_threshold: 0.8,
                        }),
                        critical: Some(CpuThresholdsConfig {
                            one_threshold: 1.0,
                            five_threshold: 1.0,
                            fifteen_threshold: 1.0,
                        }),
                    })
                    .unwrap(),
                    FakeCpuSource {
                        one_value,
                        five_value,
                        fifteen_value,
                    },
                );

                assert_eq!(
                    cpu_checker.check().unwrap(),
                    CheckResult::new(
                        String::from("cpu"),
                        CheckStatus::CRITICAL,
                        Some(format!(
                            "one: {one_value}, five: {five_value}, fifteen: {fifteen_value}"
                        )),
                    ),
                );
            }

            #[test]
            fn it_should_be_warning() {
                let one_value = 0.9;
                let five_value = 0.9;
                let fifteen_value = 0.9;

                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
                        enabled: Some(true),
                        warning: Some(CpuThresholdsConfig {
                            one_threshold: 0.8,
                            five_threshold: 0.8,
                            fifteen_threshold: 0.8,
                        }),
                        critical: Some(CpuThresholdsConfig {
                            one_threshold: 1.0,
                            five_threshold: 1.0,
                            fifteen_threshold: 1.0,
                        }),
                    })
                    .unwrap(),
                    FakeCpuSource {
                        one_value,
                        five_value,
                        fifteen_value,
                    },
                );

                assert_eq!(
                    cpu_checker.check().unwrap(),
                    CheckResult::new(
                        String::from("cpu"),
                        CheckStatus::WARNING,
                        Some(format!(
                            "one: {one_value}, five: {five_value}, fifteen: {fifteen_value}"
                        )),
                    ),
                );
            }

            #[test]
            fn it_should_be_ok() {
                let one_value = 0.7;
                let five_value = 0.7;
                let fifteen_value = 0.7;

                let cpu_checker = CpuChecker::new(
                    CpuSettings::try_from(&CpuConfig {
                        enabled: Some(true),
                        warning: Some(CpuThresholdsConfig {
                            one_threshold: 0.8,
                            five_threshold: 0.8,
                            fifteen_threshold: 0.8,
                        }),
                        critical: Some(CpuThresholdsConfig {
                            one_threshold: 1.0,
                            five_threshold: 1.0,
                            fifteen_threshold: 1.0,
                        }),
                    })
                    .unwrap(),
                    FakeCpuSource {
                        one_value,
                        five_value,
                        fifteen_value,
                    },
                );

                assert_eq!(
                    cpu_checker.check().unwrap(),
                    CheckResult::new(String::from("cpu"), CheckStatus::OK, None),
                );
            }
        }
    }
}
