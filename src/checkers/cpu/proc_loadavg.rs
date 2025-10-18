use super::CpuSource;
use std::fs;

#[derive(thiserror::Error, Debug)]
#[error("Failed to parse CPU settings: {0}")]
pub struct CpuParseError(pub String);

pub struct ProcLoadavg;

impl ProcLoadavg {
    pub fn parse_loadavg(s: &str) -> Result<(f32, f32, f32), CpuParseError> {
        let mut parts = s.split_whitespace();
        let one = parts
            .next()
            .ok_or_else(|| CpuParseError("missing 1m".into()))?;
        let five = parts
            .next()
            .ok_or_else(|| CpuParseError("missing 5m".into()))?;
        let fifteen = parts
            .next()
            .ok_or_else(|| CpuParseError("missing 15m".into()))?;

        let parse_number = |x: &str| {
            x.parse::<f32>().map_err(|e| {
                tracing::info!("Error: parse_number: {} | {}", x, e.to_string());
                CpuParseError("can't parse input".into())
            })
        };

        Ok((
            parse_number(one)?,
            parse_number(five)?,
            parse_number(fifteen)?,
        ))
    }
}

impl CpuSource for ProcLoadavg {
    fn parse_values(&self) -> Result<(f32, f32, f32), CpuParseError> {
        let s = fs::read_to_string("/proc/loadavg").map_err(|e| CpuParseError(e.to_string()))?;

        Self::parse_loadavg(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod proc_loadavg {
        use super::*;

        mod parse_loadavg {
            use super::*;

            #[test]
            fn it_should_parse_the_string() {
                assert_eq!(
                    ProcLoadavg::parse_loadavg("10 15 20").unwrap(),
                    (10.0, 15.0, 20.0),
                )
            }

            #[test]
            fn it_should_miss_1m_value() {
                let err = ProcLoadavg::parse_loadavg("").unwrap_err();

                match err {
                    CpuParseError(msg) => assert!(msg.contains("missing 1m")),
                    _ => panic!("Unexpected error type"),
                }
            }

            #[test]
            fn it_should_miss_5m_value() {
                let err = ProcLoadavg::parse_loadavg("just").unwrap_err();

                match err {
                    CpuParseError(msg) => assert!(msg.contains("missing 5m")),
                    _ => panic!("Unexpected error type"),
                }
            }

            #[test]
            fn it_should_miss_15m_value() {
                let err = ProcLoadavg::parse_loadavg("just random").unwrap_err();

                match err {
                    CpuParseError(msg) => assert!(msg.contains("missing 15m")),
                    _ => panic!("Unexpected error type"),
                }
            }

            #[test]
            fn it_should_not_parse_1m_value() {
                let err = ProcLoadavg::parse_loadavg("just random string").unwrap_err();

                match err {
                    CpuParseError(msg) => assert!(msg.contains("can't parse input")),
                    _ => panic!("Unexpected error type"),
                }
            }

            #[test]
            fn it_should_not_parse_5m_value() {
                let err = ProcLoadavg::parse_loadavg("10 random string").unwrap_err();

                match err {
                    CpuParseError(msg) => assert!(msg.contains("can't parse input")),
                    _ => panic!("Unexpected error type"),
                }
            }

            #[test]
            fn it_should_not_parse_15m_value() {
                let err = ProcLoadavg::parse_loadavg("10 15 string").unwrap_err();

                match err {
                    CpuParseError(msg) => assert!(msg.contains("can't parse input")),
                    _ => panic!("Unexpected error type"),
                }
            }
        }
    }
}
