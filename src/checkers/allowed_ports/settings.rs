use super::AllowedPortsConfig;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum WrongAllowedPortsSettingsError {
    #[error("Empty processsettings for allowed ports")]
    EmptyProcessesSettings,

    #[error("Invalid port bounds for process: {0}")]
    InvalidPortBounds(String),

    #[error("Invalid port value for process: {0}")]
    InvalidPortValue(String),
}

pub struct PortBounds {
    pub min: u16,
    pub max: u16,
}

trait TrimAll {
    fn trim_all(&self) -> String;
}

impl TrimAll for &str {
    fn trim_all(&self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

impl TryFrom<&str> for PortBounds {
    type Error = WrongAllowedPortsSettingsError;

    fn try_from(port_bounds: &str) -> Result<Self, Self::Error> {
        let trimmed_port_bounds = port_bounds.trim_all();
        let parts = trimmed_port_bounds.split('-').collect::<Vec<&str>>();

        if parts.len() == 1 {
            let port = parts[0].parse::<u16>().map_err(|_| {
                WrongAllowedPortsSettingsError::InvalidPortValue(port_bounds.to_string())
            })?;
            return Ok(PortBounds {
                min: port,
                max: port,
            });
        }

        if parts.len() == 2 {
            let min = parts[0].parse::<u16>().map_err(|_| {
                WrongAllowedPortsSettingsError::InvalidPortValue(port_bounds.to_string())
            })?;
            let max = parts[1].parse::<u16>().map_err(|_| {
                WrongAllowedPortsSettingsError::InvalidPortValue(port_bounds.to_string())
            })?;
            return Ok(PortBounds { min, max });
        }

        Err(WrongAllowedPortsSettingsError::InvalidPortBounds(
            port_bounds.to_string(),
        ))
    }
}

pub struct AllowedPortsSettings {
    pub enabled: bool,
    pub processes: HashMap<String, Vec<PortBounds>>,
}

impl TryFrom<&AllowedPortsConfig> for AllowedPortsSettings {
    type Error = WrongAllowedPortsSettingsError;

    fn try_from(allowed_ports_config: &AllowedPortsConfig) -> Result<Self, Self::Error> {
        if !allowed_ports_config.enabled {
            return Ok(AllowedPortsSettings {
                enabled: false,
                processes: HashMap::new(),
            });
        }

        if allowed_ports_config.processes.len() == 0 {
            return Err(WrongAllowedPortsSettingsError::EmptyProcessesSettings);
        }

        let processes: HashMap<String, Vec<PortBounds>> = allowed_ports_config
            .processes
            .iter()
            .map(|process_data| {
                let bounds = process_data
                    .ports
                    .iter()
                    .map(|s| PortBounds::try_from(s.as_str()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok((process_data.owner.clone(), bounds))
            })
            .collect::<Result<_, _>>()?;

        Ok(AllowedPortsSettings {
            enabled: true,
            processes,
        })
    }
}
