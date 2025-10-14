use crate::domain::DiskSnapshot;
use crate::infra::ParseError;

pub trait CpuSource: Send + Sync {
    fn parse_values(&self) -> Result<(f32, f32, f32), ParseError>;
}

pub trait RamSource: Send + Sync {
    fn parse_values(&self) -> Result<f32, ParseError>;
}

pub trait DiskUsageSource: Send + Sync {
    fn parse_values(&self) -> Result<Vec<DiskSnapshot>, ParseError>;
}
