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
