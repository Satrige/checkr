use super::super::errors::ParseError;

pub fn parse_loadavg(s: &str) -> Result<(f32, f32, f32), ParseError> {
    // /proc/loadavg starts with: "<1m> <5m> <15m> ..."
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
