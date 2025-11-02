mod ss_parser;
mod ss_runner;

use std::fmt;

use crate::checkers::allowed_ports::OpenPortsSource;
use ss_parser::SsParser;
use ss_runner::SsRunner;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PortEntry {
    pub owner: String,
    pub port: u16,
}

impl From<(String, u16)> for PortEntry {
    fn from((owner, port): (String, u16)) -> Self {
        Self { owner, port }
    }
}

impl fmt::Display for PortEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]: {}", self.owner, self.port)
    }
}

pub struct SsSource;

impl OpenPortsSource for SsSource {
    fn parse_values(&self) -> anyhow::Result<Vec<PortEntry>> {
        let ss_output = SsRunner::run()?;
        let result = SsParser::parse_ss_info(ss_output)?
            .into_iter()
            .map(PortEntry::from)
            .collect::<Vec<PortEntry>>();
        Ok(result)
    }
}
