use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AllowedPortsConfig {
    pub enabled: Option<bool>,
    pub ports: Option<Vec<u16>>,
}
