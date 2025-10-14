#[derive(thiserror::Error, Debug)]
pub enum WrongSettingsError {
    #[error("Wrong CPU settings: {0}")]
    WrongCpuSettingsError(String),

    #[error("Wrong RAM settings: {0}")]
    WrongRamSettingsError(String),

    #[error("Wrong Disk Usage settings: {0}")]
    WrongDiskUsageSettingsError(String),
}
