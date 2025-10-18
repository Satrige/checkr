use super::RamConfig;

#[derive(thiserror::Error, Debug)]
#[error("Wrong RAM settings: {0}")]
pub struct WrongRamSettingsError(pub String);

pub struct RamSettings {
    pub enabled: bool,
    pub warning_threshold: f32,
    pub critical_threshold: f32,
}

impl Default for RamSettings {
    fn default() -> Self {
        RamSettings {
            enabled: true,
            warning_threshold: 0.0,
            critical_threshold: 0.0,
        }
    }
}

impl TryFrom<&RamConfig> for RamSettings {
    type Error = WrongRamSettingsError;

    fn try_from(ram_config: &RamConfig) -> Result<Self, Self::Error> {
        // The case the check is explicitly disabled
        if let Some(ram_check_enabled) = ram_config.enabled
            && !ram_check_enabled
        {
            return Ok(RamSettings {
                enabled: false,
                warning_threshold: 0.0,
                critical_threshold: 0.0,
            });
        }

        // The case the check is explicitly enabled but the thresholds are absent
        if ram_config.warning_threshold.is_none() || ram_config.critical_threshold.is_none() {
            return Err(WrongRamSettingsError(
                "The thresholds are not specified".into(),
            ));
        }

        Ok(RamSettings {
            enabled: true,
            warning_threshold: ram_config.warning_threshold.unwrap(),
            critical_threshold: ram_config.critical_threshold.unwrap(),
        })
    }
}
