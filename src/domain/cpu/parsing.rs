use super::super::errors::ParseError;

pub fn parse_loadavg(s: &str) -> Result<(f32, f32, f32), ParseError> {
    let mut parts = s.split_whitespace();
    let one = parts
        .next()
        .ok_or_else(|| ParseError::CpuParseError("missing 1m".into()))?;
    let five = parts
        .next()
        .ok_or_else(|| ParseError::CpuParseError("missing 5m".into()))?;
    let fifteen = parts
        .next()
        .ok_or_else(|| ParseError::CpuParseError("missing 15m".into()))?;

    let parse_number = |x: &str| {
        x.parse::<f32>()
            .map_err(|e| ParseError::CpuParseError(e.to_string()))
    };
    Ok((
        parse_number(one)?,
        parse_number(five)?,
        parse_number(fifteen)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_loadavg {
        use super::*;

        #[test]
        fn it_should_parse_the_string() {
            assert_eq!(parse_loadavg("10 15 20").unwrap(), (10.0, 15.0, 20.0),)
        }

        #[test]
        fn it_should_not_parse_1m_value() {
            let err = parse_loadavg("just random sting").unwrap_err();

            match err {
                ParseError::CpuParseError(msg) => assert!(msg.contains("missing 1m")),
                _ => panic!("Unexpected error type"),
            }
        }

        #[test]
        fn it_should_not_parse_5m_value() {
            let err = parse_loadavg("10 random sting").unwrap_err();

            match err {
                ParseError::CpuParseError(msg) => assert!(msg.contains("missing 5m")),
                _ => panic!("Unexpected error type"),
            }
        }

        #[test]
        fn it_should_not_parse_15m_value() {
            let err = parse_loadavg("10 15 sting").unwrap_err();

            match err {
                ParseError::CpuParseError(msg) => assert!(msg.contains("missing 15m")),
                _ => panic!("Unexpected error type"),
            }
        }
    }
}
