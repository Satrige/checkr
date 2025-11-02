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
            .filter(|entry| match self.settings.processes.get(&entry.owner) {
                None => true,
                Some(port_bounds) => port_bounds
                    .iter()
                    .find(|bound| bound.min <= entry.port && entry.port <= bound.max)
                    .is_none(),
            })
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
