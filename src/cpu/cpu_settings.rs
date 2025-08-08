use crate::{cpu::cpu_config::CpuConfig, models::errors::WrongSettingsError};

pub struct CpuThresholdSettings {
    pub one_threshold: f32,
    pub five_threshold: f32,
    pub fifteen_threshold: f32,
}

impl CpuThresholdSettings {
    fn new(one_threshold: f32, five_threshold: f32, fifteen_threshold: f32) -> Self {
        CpuThresholdSettings {
            one_threshold,
            five_threshold,
            fifteen_threshold,
        }
    }
}

pub struct CpuSettings {
    pub enabled: bool,
    pub warning: CpuThresholdSettings,
    pub critical: CpuThresholdSettings,
}

impl Default for CpuSettings {
    fn default() -> Self {
        CpuSettings {
            enabled: true,
            warning: CpuThresholdSettings::new(0.5, 0.5, 0.5),
            critical: CpuThresholdSettings::new(1.0, 1.0, 1.0),
        }
    }
}

impl TryFrom<&CpuConfig> for CpuSettings {
    type Error = WrongSettingsError;

    fn try_from(cpu_config: &CpuConfig) -> Result<Self, Self::Error> {
        // The case the check is explicitly disabled
        if let Some(cpu_check_enabled) = cpu_config.enabled
            && !cpu_check_enabled
        {
            return Ok(CpuSettings {
                enabled: false,
                warning: CpuThresholdSettings::new(0.0, 0.0, 0.0),
                critical: CpuThresholdSettings::new(0.0, 0.0, 0.0),
            });
        }

        // The case the check is explicitly enabled but the thresholds are absent
        if cpu_config.warning.is_none() || cpu_config.critical.is_none() {
            return Err(WrongSettingsError::WrongCpuSettingsError(
                "The thresholds are not specified".into(),
            ));
        }

        let warning_thresholds = cpu_config.warning.as_ref().unwrap();
        let critical_thresholds = cpu_config.critical.as_ref().unwrap();

        Ok(CpuSettings {
            enabled: true,
            warning: CpuThresholdSettings::new(
                warning_thresholds.one_threshold,
                warning_thresholds.five_threshold,
                warning_thresholds.fifteen_threshold,
            ),
            critical: CpuThresholdSettings::new(
                critical_thresholds.one_threshold,
                critical_thresholds.five_threshold,
                critical_thresholds.fifteen_threshold,
            ),
        })
    }
}
