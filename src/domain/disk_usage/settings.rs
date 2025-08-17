use crate::config::DiskUsageConfig;

use super::super::errors::WrongSettingsError;

pub struct DiskUsageSettings {
    pub enabled: bool,
    pub warning_threshold: f32,
    pub critical_threshold: f32,
}

impl Default for DiskUsageSettings {
    fn default() -> Self {
        DiskUsageSettings {
            enabled: true,
            warning_threshold: 70.0,
            critical_threshold: 80.0,
        }
    }
}

impl TryFrom<&DiskUsageConfig> for DiskUsageSettings {
    type Error = WrongSettingsError;

    fn try_from(disk_usage_config: &DiskUsageConfig) -> Result<Self, Self::Error> {
        // The case the check is explicitly disabled
        if let Some(disk_usage_check_enabled) = disk_usage_config.enabled
            && !disk_usage_check_enabled
        {
            return Ok(DiskUsageSettings {
                enabled: false,
                warning_threshold: 0.0,
                critical_threshold: 0.0,
            });
        }

        // The case the check is explicitly enabled but the thresholds are absent
        if disk_usage_config.warning_threshold.is_none()
            || disk_usage_config.critical_threshold.is_none()
        {
            return Err(WrongSettingsError::WrongDiskUsageSettingsError(
                "The thresholds are not specified".into(),
            ));
        }

        Ok(DiskUsageSettings {
            enabled: true,
            warning_threshold: disk_usage_config.warning_threshold.unwrap(),
            critical_threshold: disk_usage_config.critical_threshold.unwrap(),
        })
    }
}
