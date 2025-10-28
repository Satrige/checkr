use super::AllowedPortsConfig;

#[derive(thiserror::Error, Debug)]
#[error("Wrong settings for allowed ports: {0}")]
pub struct WrongAllowedPortsSettingsError(pub String);

pub struct AllowedPortsSettings {
    pub enabled: bool,
    pub ports: Vec<u16>,
}

impl TryFrom<&AllowedPortsConfig> for AllowedPortsSettings {
    type Error = WrongAllowedPortsSettingsError;

    fn try_from(allowed_ports_config: &AllowedPortsConfig) -> Result<Self, Self::Error> {
        if let Some(is_enabled) = &allowed_ports_config.enabled
            && !is_enabled
        {
            return Ok(AllowedPortsSettings {
                enabled: false,
                ports: Vec::new(),
            });
        }

        Ok(AllowedPortsSettings {
            enabled: true,
            ports: allowed_ports_config.ports.as_ref().unwrap().clone(),
        })
    }
}
