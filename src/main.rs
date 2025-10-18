mod checkers;
mod checkers_factory;
mod config;
mod utils;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load()?;

    tracing_subscriber::fmt()
        .with_max_level(config.log_level.as_ref().unwrap_or(&utils::LogLevel::ERROR))
        .init();

    let checkers = checkers_factory::build_checkers(&config)?;
    web::start_server(config, checkers).await;

    Ok(())
}
