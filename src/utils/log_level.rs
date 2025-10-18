use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    TRACE,
}

impl From<&LogLevel> for LevelFilter {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::DEBUG => LevelFilter::DEBUG,
            LogLevel::INFO => LevelFilter::INFO,
            LogLevel::WARN => LevelFilter::WARN,
            LogLevel::ERROR => LevelFilter::ERROR,
            LogLevel::TRACE => LevelFilter::TRACE,
        }
    }
}
