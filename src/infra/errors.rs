#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Cpu parse error: {0}")]
    CpuParseError(String),

    #[error("Ram parse error: {0}")]
    RamParseError(String),

    #[error("Disk usage parse error: {0}")]
    DiskUsageError(String),
}
