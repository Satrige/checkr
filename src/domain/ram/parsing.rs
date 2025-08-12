use super::super::errors::ParseError;

fn extract_kb_value(line: &str) -> f32 {
    line.split_whitespace()
        .nth(1)
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.0)
}

pub fn parse_meminfo(meminfo: &str) -> Result<f32, ParseError> {
    let mut mem_total = 0.0;
    let mut mem_available = 0.0;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            mem_total = extract_kb_value(line);
        } else if line.starts_with("MemAvailable:") {
            mem_available = extract_kb_value(line);
        }
    }

    if mem_total == 0.0 {
        return Err(ParseError::RamParseError("MemTotal missing".into()));
    }

    Ok((mem_total - mem_available) * 100.0 / mem_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extract_kb_value {
        use super::*;

        #[test]
        fn it_should_correctly_extract_kb_value() {
            assert_eq!(extract_kb_value("MemTotal:        1921988 kB"), 1921988.0,);
        }

        #[test]
        fn it_should_fall_back_to_zero_value() {
            assert_eq!(
                extract_kb_value("Just the random string without kb info"),
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

            assert_eq!(parse_meminfo(meminfo).unwrap(), 50.0,)
        }

        #[test]
        fn if_should_not_be_able_to_parse_ram_usage() {
            let meminfo = "Just the random string";

            let result = parse_meminfo(meminfo);

            let err = result.unwrap_err();
            match err {
                ParseError::RamParseError(msg) => assert!(msg.contains("MemTotal")),
                _ => panic!("Unexpected error type"),
            }
        }
    }
}
