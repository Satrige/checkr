use crate::domain::{errors::ParseError, parse_meminfo, ports::RamSource};
use std::fs;

pub struct ProcMeminfo;

impl RamSource for ProcMeminfo {
    fn parse_values(&self) -> Result<f32, ParseError> {
        let s = fs::read_to_string("/proc/meminfo")
            .map_err(|e| ParseError::RamParseError(e.to_string()))?;

        parse_meminfo(&s)
    }
}
