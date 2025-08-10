use crate::domain::errors::ParseError;

pub trait CpuSource: Send + Sync {
    fn read_load(&self) -> Result<(f32, f32, f32), ParseError>;
}
