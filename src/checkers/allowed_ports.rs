mod config;
mod infra;
mod settings;

use super::{CheckResult, CheckStatus, Checker};
pub use config::*;
pub use infra::*;
pub use settings::AllowedPortsSettings;

pub trait OpenPortsSource: Send + Sync {
    fn parse_values(&self) -> anyhow::Result<Vec<PortEntry>>;
}

pub struct AllowedPortsChecker<S: OpenPortsSource> {
    settings: AllowedPortsSettings,
    name: String,
    source: S,
}

impl<S: OpenPortsSource> AllowedPortsChecker<S> {
    pub fn new(settings: AllowedPortsSettings, source: S) -> Self {
        AllowedPortsChecker {
            settings,
            name: "allowed_ports".to_string(),
            source,
        }
    }

    fn join_ports(&self, ports: &[u16]) -> String {
        ports
            .iter()
            .map(u16::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn format_response(&self, open_ports: Vec<PortEntry>) -> String {
        open_ports
            .into_iter()
            .map(|open_port| open_port.port().to_string())
            .collect::<Vec<String>>()
            .join("; ")
    }
}

impl<S: OpenPortsSource> Checker for AllowedPortsChecker<S> {
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

        let filtered_open_ports: Vec<PortEntry> = self
            .source
            .parse_values()?
            .into_iter()
            .filter(|entry| !self.settings.ports.contains(&entry.port()))
            .collect();

        Ok(CheckResult::new(
            self.name.clone(),
            if filtered_open_ports.is_empty() {
                CheckStatus::OK
            } else {
                CheckStatus::CRITICAL
            },
            if filtered_open_ports.is_empty() {
                None
            } else {
                Some(self.format_response(filtered_open_ports))
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Clone)]
    struct FakeOpenPortsSource {
        ports: Vec<PortEntry>,
    }

    impl OpenPortsSource for FakeOpenPortsSource {
        fn parse_values(&self) -> anyhow::Result<Vec<PortEntry>> {
            Ok(self.ports.clone())
        }
    }

    mod settings_try_from {
        use super::*;

        #[test]
        fn it_should_be_disabled_when_config_disabled() {
            let cfg = AllowedPortsConfig {
                enabled: Some(false),
                ports: None,
            };
            let s = AllowedPortsSettings::try_from(&cfg).unwrap();
            assert_eq!(s.enabled, false);
            assert!(s.ports.is_empty());
        }

        #[test]
        fn it_should_enable_and_take_ports_from_config() {
            let cfg = AllowedPortsConfig {
                enabled: Some(true),
                ports: Some(vec![22, 80]),
            };
            let s = AllowedPortsSettings::try_from(&cfg).unwrap();
            assert_eq!(s.enabled, true);
            assert_eq!(s.ports, vec![22, 80]);
        }

        #[test]
        #[should_panic]
        fn it_should_panic_if_enabled_but_ports_missing() {
            let cfg = AllowedPortsConfig {
                enabled: Some(true),
                ports: None,
            };
            let _ = AllowedPortsSettings::try_from(&cfg).unwrap();
        }
    }

    mod checker_check {
        use super::*;

        #[test]
        fn it_should_be_disabled() {
            let settings = AllowedPortsSettings {
                enabled: false,
                ports: Vec::new(),
            };
            let checker = AllowedPortsChecker::new(settings, FakeOpenPortsSource::default());
            assert_eq!(
                checker.check().unwrap(),
                CheckResult::new(String::from("allowed_ports"), CheckStatus::DISABLED, None)
            );
        }

        #[test]
        fn it_should_be_ok_when_all_open_ports_are_allowed() {
            let settings = AllowedPortsSettings {
                enabled: true,
                ports: vec![22, 80],
            };
            let source = FakeOpenPortsSource {
                ports: vec![PortEntry(22), PortEntry(80)],
            };
            let checker = AllowedPortsChecker::new(settings, source);
            assert_eq!(
                checker.check().unwrap(),
                CheckResult::new(String::from("allowed_ports"), CheckStatus::OK, None)
            );
        }

        #[test]
        fn it_should_be_critical_when_any_open_port_not_in_allowed_list() {
            let settings = AllowedPortsSettings {
                enabled: true,
                ports: vec![22, 80],
            };
            let source = FakeOpenPortsSource {
                ports: vec![PortEntry(22), PortEntry(443)],
            };
            let checker = AllowedPortsChecker::new(settings, source);
            assert_eq!(
                checker.check().unwrap(),
                CheckResult::new(
                    String::from("allowed_ports"),
                    CheckStatus::CRITICAL,
                    Some(String::from("443"))
                )
            );
        }

        #[test]
        fn it_should_list_all_disallowed_ports_in_message_in_input_order() {
            let settings = AllowedPortsSettings {
                enabled: true,
                ports: vec![22, 80],
            };
            let source = FakeOpenPortsSource {
                ports: vec![PortEntry(22), PortEntry(8080), PortEntry(3306)],
            };
            let checker = AllowedPortsChecker::new(settings, source);
            assert_eq!(
                checker.check().unwrap(),
                CheckResult::new(
                    String::from("allowed_ports"),
                    CheckStatus::CRITICAL,
                    Some(String::from("8080; 3306"))
                )
            );
        }
    }
}
