use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CpuThresholdsConfig {
    pub one_threshold: f32,
    pub five_threshold: f32,
    pub fifteen_threshold: f32,
}

#[derive(Debug, Deserialize)]
pub struct CpuConfig {
    pub enabled: Option<bool>,
    pub warning: Option<CpuThresholdsConfig>,
    pub critical: Option<CpuThresholdsConfig>,
}
