mod app;
mod config;
mod domain;
mod infra;
mod web;

use app::build::build_checkers;

use crate::infra::log_level::LogLevel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load()?;

    tracing_subscriber::fmt()
        .with_max_level(config.log_level.as_ref().unwrap_or(&LogLevel::ERROR))
        .init();

    let checkers = build_checkers(&config)?;
    web::start_server(config, checkers).await;

    Ok(())
}
