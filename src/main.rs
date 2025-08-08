use axum::Router;
use clap::Parser;

mod cpu;
mod ram;

mod config;
mod helpers;
mod models;
mod routes;

use config::{AppConfig, Args};
use helpers::get_checkers::get_checkers;
use models::log_level::LogLevel;
use routes::HealthRouters;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = AppConfig::from_file(&args.config)?;

    tracing_subscriber::fmt()
        .with_max_level(config.log_level.as_ref().unwrap_or(&LogLevel::ERROR))
        .init();

    tracing::info!("Started checkr on port: {}", config.port);

    let checkers = get_checkers(&config)?;

    let app = Router::new().nest("/health", HealthRouters::new(checkers).get_rountes());

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
