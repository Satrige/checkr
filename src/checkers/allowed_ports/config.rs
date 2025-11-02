use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PortOwnerInfo {
    pub owner: String,
    pub ports: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AllowedPortsConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub processes: Vec<PortOwnerInfo>,
}
