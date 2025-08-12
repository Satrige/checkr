#[derive(thiserror::Error, Debug)]
pub(crate) enum ParseError {
    #[error("Cpu parse error: {0}")]
    CpuParseError(String),

    #[error("Ram parse error: {0}")]
    RamParseError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum WrongSettingsError {
    #[error("Wrong CPU settings: {0}")]
    WrongCpuSettingsError(String),

    #[error("Wrong RAM settings: {0}")]
    WrongRamSettingsError(String),
}
