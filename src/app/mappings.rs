use crate::{
    config::cpu_config::CpuConfig, cpu::cpu_settings::CpuSettings,
    models::errors::WrongSettingsError,
};

pub fn cpu_settings_from(cpu_config: &CpuConfig) -> Result<CpuSettings, WrongSettingsError> {}
