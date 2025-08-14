use clap::Parser;
use serde::Deserialize;
use serde_json;
use std::fs;

mod cpu_config;
mod errors;
mod ram_config;

use crate::infra::log_level::LogLevel;
pub use cpu_config::*;
use errors::ConfigError;
pub use ram_config::*;

#[derive(Parser, Debug)]
#[command(name = "checkr")]
struct Args {
    #[arg(short, long, default_value = "config.json")]
    config: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub log_level: Option<LogLevel>,

    pub cpu: Option<CpuConfig>,
    pub ram: Option<RamConfig>,
}

impl AppConfig {
    fn from_file(path: &str) -> Result<Self, ConfigError> {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| ConfigError::ParseConfigError(e.to_string())),
            Err(read_error) => Err(ConfigError::ReadConfigError(read_error.to_string())),
        }
    }
}

pub fn load() -> Result<AppConfig, ConfigError> {
    let args = Args::parse();

    AppConfig::from_file(&args.config)
}
