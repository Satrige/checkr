use crate::domain::{cpu::parsing::parse_loadavg, errors::ParseError, ports::CpuSource};
use std::fs;

pub struct ProcLoadavg;

impl CpuSource for ProcLoadavg {
    fn read_load(&self) -> Result<(f32, f32, f32), ParseError> {
        let s = fs::read_to_string("/proc/loadavg")
            .map_err(|e| ParseError::CpuParseError(e.to_string()))?;

        parse_loadavg(&s)
    }
}
