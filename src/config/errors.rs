use thiserror;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ConfigError {
    #[error("Read config error: {0}")]
    ReadConfigError(String),

    #[error("Parse config error: {0}")]
    ParseConfigError(String),
}
