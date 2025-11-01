use regex::Regex;
use std::collections::HashSet;

#[derive(thiserror::Error, Debug)]
pub enum SsParseError {
    #[error("Too few fields in ss output: {0}")]
    TooFewFields(String),

    #[error("Malformed local addr in ss output: {0}")]
    BadLocalAddr(String),

    #[error("Malformed port in ss output: {0}")]
    InvalidPort(String),
}

pub struct SsParser;

impl SsParser {
    fn parse_ss_line(line: &str) -> Result<Vec<(String, u16)>, SsParseError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return Err(SsParseError::TooFewFields(line.to_string()));
        }

        let local_idx = (4..parts.len())
            .find(|&i| parts[i].contains(':') && !parts[i].starts_with("users:"))
            .ok_or_else(|| SsParseError::BadLocalAddr(line.to_string()))?;
        let local_addr = parts[local_idx];

        let port_str = local_addr
            .rsplitn(2, ':')
            .next()
            .ok_or_else(|| SsParseError::BadLocalAddr(local_addr.to_string()))?;
        let port: u16 = port_str
            .parse()
            .map_err(|_| SsParseError::InvalidPort(port_str.to_string()))?;

        // from users:(("name",pid=...,fd=...),("other",pid=...,fd=...))
        let re = Regex::new(r#"\("([^"]+)",pid=\d+"#).unwrap();
        let mut owners: HashSet<String> = HashSet::new();
        for captures in re.captures_iter(line) {
            if let Some(name) = captures.get(1) {
                owners.insert(name.as_str().to_string());
            }
        }

        if owners.is_empty() {
            return Ok(vec![]);
        }

        Ok(owners
            .into_iter()
            .map(|owner| (owner.clone(), port))
            .collect())
    }

    pub fn parse_ss_info(ss_output: String) -> anyhow::Result<Vec<(String, u16)>> {
        let mut result = vec![];

        for line in ss_output.lines() {
            if line == "" {
                continue;
            }
            let port_entries = Self::parse_ss_line(line)?;
            result.extend(port_entries);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_ipv4_single_owner() {
        let line = r#"tcp LISTEN 0 128 0.0.0.0:22 0.0.0.0:* users:(("sshd",pid=743,fd=3))"#;
        let entries = SsParser::parse_ss_line(line).unwrap();
        assert_eq!(entries, vec![("sshd".to_string(), 22)]);
    }

    #[test]
    fn parse_line_ipv6_single_owner() {
        let line = r#"tcp LISTEN 0 128 [::]:80 [::]:* users:(("nginx",pid=100,fd=7))"#;
        let entries = SsParser::parse_ss_line(line).unwrap();
        assert_eq!(entries, vec![("nginx".to_string(), 80)]);
    }

    #[test]
    fn parse_line_wildcard_host() {
        let line = r#"udp UNCONN 0 0 *:53 *:* users:(("named",pid=200,fd=9))"#;
        let entries = SsParser::parse_ss_line(line).unwrap();
        assert_eq!(entries, vec![("named".to_string(), 53)]);
    }

    #[test]
    fn parse_line_multiple_owners() {
        let line = r#"tcp LISTEN 0 128 0.0.0.0:22 0.0.0.0:* users:(("sshd",pid=743,fd=3),("systemd",pid=1,fd=92))"#;
        let mut entries = SsParser::parse_ss_line(line).unwrap();
        entries.sort(); // order from HashSet is unspecified
        assert_eq!(
            entries,
            vec![("sshd".to_string(), 22), ("systemd".to_string(), 22)]
        );
    }

    #[test]
    fn parse_line_no_users_returns_empty() {
        let line = r#"udp UNCONN 0 0 0.0.0.0:68 0.0.0.0:* users:()"#;
        let entries = SsParser::parse_ss_line(line).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn error_too_few_fields() {
        let line = "tcp LISTEN 0 128"; // clearly malformed
        let err = SsParser::parse_ss_line(line).unwrap_err();
        matches!(err, SsParseError::TooFewFields(_));
    }

    #[test]
    fn error_bad_local_addr_when_missing_port() {
        // no ':' in suspected local address field
        let line = r#"tcp LISTEN 0 128 localhost 0.0.0.0:* users:(("sshd",pid=1,fd=3))"#;
        let err = SsParser::parse_ss_line(line).unwrap_err();
        matches!(err, SsParseError::BadLocalAddr(_));
    }

    #[test]
    fn error_invalid_port() {
        let line = r#"tcp LISTEN 0 128 0.0.0.0:notaport 0.0.0.0:* users:(("sshd",pid=1,fd=3))"#;
        let err = SsParser::parse_ss_line(line).unwrap_err();
        matches!(err, SsParseError::InvalidPort(_));
    }

    #[test]
    fn parse_info_aggregates_multiple_lines() {
        let input = r#"
tcp LISTEN 0 128 0.0.0.0:22 0.0.0.0:* users:(("sshd",pid=743,fd=3),("systemd",pid=1,fd=92))
tcp LISTEN 0 128 [::]:80 [::]:* users:(("nginx",pid=100,fd=7))
udp UNCONN 0 0 0.0.0.0:68 0.0.0.0:* users:()
"#
        .to_string();

        let mut entries = SsParser::parse_ss_info(input).unwrap();
        entries.sort();

        assert_eq!(
            entries,
            vec![
                ("nginx".to_string(), 80),
                ("sshd".to_string(), 22),
                ("systemd".to_string(), 22),
            ]
        );
    }
}
