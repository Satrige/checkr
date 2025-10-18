#[derive(thiserror::Error, Debug)]
pub enum CpuError {
    #[error("Wrong CPU settings: {0}")]
    WrongCpuSettingsError(String),
}
