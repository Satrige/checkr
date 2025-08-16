use crate::domain::{DiskSnapshot, ports::DiskUsageSource};

pub struct ProcSelfMounts;

impl DiskUsageSource for ProcSelfMounts {
    fn parse_values(&self) -> Result<Vec<DiskSnapshot>, crate::domain::errors::ParseError> {
        Ok(Vec::new())
    }
}
