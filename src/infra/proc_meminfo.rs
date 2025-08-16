use crate::domain::{errors::ParseError, ports::RamSource};
use std::fs;

pub struct ProcMeminfo;

impl ProcMeminfo {
    fn extract_kb_value(line: &str) -> f32 {
        line.split_whitespace()
            .nth(1)
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    fn parse_meminfo(meminfo: &str) -> Result<f32, ParseError> {
        let mut mem_total = 0.0;
        let mut mem_available = 0.0;

        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total = Self::extract_kb_value(line);
            } else if line.starts_with("MemAvailable:") {
                mem_available = Self::extract_kb_value(line);
            }
        }

        if mem_total == 0.0 {
            return Err(ParseError::RamParseError("MemTotal missing".into()));
        }

        Ok((mem_total - mem_available) * 100.0 / mem_total)
    }
}

impl RamSource for ProcMeminfo {
    fn parse_values(&self) -> Result<f32, ParseError> {
        let s = fs::read_to_string("/proc/meminfo")
            .map_err(|e| ParseError::RamParseError(e.to_string()))?;

        Self::parse_meminfo(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod proc_meminfo {
        use super::*;

        mod extract_kb_value {
            use super::*;

            #[test]
            fn it_should_correctly_extract_kb_value() {
                assert_eq!(
                    ProcMeminfo::extract_kb_value("MemTotal:        1921988 kB"),
                    1921988.0,
                );
            }

            #[test]
            fn it_should_fall_back_to_zero_value() {
                assert_eq!(
                    ProcMeminfo::extract_kb_value("Just the random string without kb info"),
                    0.0,
                );
            }
        }

        mod parse_meminfo {
            use super::*;

            #[test]
            fn it_should_correctly_parse_ram_usage_percent() {
                let meminfo = "\
MemTotal:       1000 kB
MemAvailable:    500 kB
            ";

                assert_eq!(ProcMeminfo::parse_meminfo(meminfo).unwrap(), 50.0,)
            }

            #[test]
            fn if_should_not_be_able_to_parse_ram_usage() {
                let meminfo = "Just the random string";

                let result = ProcMeminfo::parse_meminfo(meminfo);

                let err = result.unwrap_err();
                match err {
                    ParseError::RamParseError(msg) => assert!(msg.contains("MemTotal")),
                    _ => panic!("Unexpected error type"),
                }
            }
        }
    }
}
