use clap::Parser;
use serde::Deserialize;
use serde_json;
use std::fs;

use crate::{
    cpu::cpu_config::CpuConfig,
    models::{errors::ConfigError, log_level::LogLevel},
    ram::ram_config::RamConfig,
};

#[derive(Parser, Debug)]
#[command(name = "checkr")]
pub struct Args {
    #[arg(short, long, default_value = "config.json")]
    pub config: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub log_level: Option<LogLevel>,

    pub cpu: Option<CpuConfig>,
    pub ram: Option<RamConfig>,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| ConfigError::ParseConfigError(e.to_string())),
            Err(read_error) => Err(ConfigError::ReadConfigError(read_error.to_string())),
        }
    }
}
