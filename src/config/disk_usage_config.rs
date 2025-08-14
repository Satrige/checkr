use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DiskUsageConfig {
    pub enabled: Option<bool>,
    pub warning_threshold: Option<f32>,
    pub critical_threshold: Option<f32>,
}
