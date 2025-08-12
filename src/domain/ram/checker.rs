use super::super::{
    checker::{CheckResult, CheckStatus, Checker},
    ports::RamSource,
};

use super::RamSettings;

pub struct RamChecker<S: RamSource> {
    settings: RamSettings,
    name: String,
    source: S,
}

impl<S: RamSource> RamChecker<S> {
    pub fn new(settings: RamSettings, source: S) -> Self {
        RamChecker {
            settings,
            name: "ram".to_string(),
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

impl<S: RamSource> Checker for RamChecker<S> {
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

        let current_value = self.source.parse_values()?;

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
    use super::super::super::errors::ParseError;
    use super::*;

    struct FakeRamSource {
        value: f32,
    }

    impl Default for FakeRamSource {
        fn default() -> Self {
            FakeRamSource { value: 0.0 }
        }
    }

    impl RamSource for FakeRamSource {
        fn parse_values(&self) -> Result<f32, ParseError> {
            Ok(self.value)
        }
    }

    mod ram_checker {
        use super::*;
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
                    FakeRamSource::default(),
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
                    FakeRamSource::default(),
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
                    FakeRamSource::default(),
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
                    FakeRamSource::default(),
                );

                assert_eq!(ram_checker.is_critical(89.0), false);
            }
        }

        mod check {
            use super::*;
            use crate::config::ram_config::RamConfig;

            #[test]
            fn it_should_be_disabled() {
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(false),
                        warning_threshold: None,
                        critical_threshold: None,
                    })
                    .unwrap(),
                    FakeRamSource::default(),
                );

                assert_eq!(
                    ram_checker.check().unwrap(),
                    CheckResult::new(String::from("ram"), CheckStatus::DISABLED, None),
                );
            }

            #[test]
            fn it_should_be_critical() {
                let current_value = 91.0;
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                    FakeRamSource {
                        value: current_value,
                    },
                );

                assert_eq!(
                    ram_checker.check().unwrap(),
                    CheckResult::new(
                        String::from("ram"),
                        CheckStatus::CRITICAL,
                        Some(format!("usage: {current_value}%"))
                    ),
                );
            }

            #[test]
            fn it_should_be_warning() {
                let current_value = 81.0;
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                    FakeRamSource {
                        value: current_value,
                    },
                );

                assert_eq!(
                    ram_checker.check().unwrap(),
                    CheckResult::new(
                        String::from("ram"),
                        CheckStatus::WARNING,
                        Some(format!("usage: {current_value}%"))
                    ),
                );
            }

            #[test]
            fn it_should_be_ok() {
                let current_value = 79.0;
                let ram_checker = RamChecker::new(
                    RamSettings::try_from(&RamConfig {
                        enabled: Some(true),
                        warning_threshold: Some(80.0),
                        critical_threshold: Some(90.0),
                    })
                    .unwrap(),
                    FakeRamSource {
                        value: current_value,
                    },
                );

                assert_eq!(
                    ram_checker.check().unwrap(),
                    CheckResult::new(String::from("ram"), CheckStatus::OK, None),
                );
            }
        }
    }
}
