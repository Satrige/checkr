use crate::domain::errors::ParseError;

pub trait CpuSource: Send + Sync {
    fn parse_values(&self) -> Result<(f32, f32, f32), ParseError>;
}

pub trait RamSource: Send + Sync {
    fn parse_values(&self) -> Result<f32, ParseError>;
}
