#[derive(thiserror::Error, Debug)]
pub enum CheckError {
    #[error("Cpu check error: {0}")]
    CpuCheckError(String),

    #[error("Ram check error: {0}")]
    RamCheckError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum WrongSettingsError {
    #[error("Wrong CPU settings: {0}")]
    WrongCpuSettingsError(String),

    #[error("Wrong RAM settings: {0}")]
    WrongRamSettingsError(String),
}
