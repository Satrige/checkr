use std::sync::Arc;

use super::routes::health::HealthRouters;
use crate::checkers::Checker;
use crate::config::AppConfig;
use axum::Router;

pub async fn start_server(config: AppConfig, checkers: Vec<Arc<dyn Checker + Send + Sync>>) {
    let app = Router::new().nest("/health", HealthRouters::new(checkers).get_routes());

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Started checkr on port: {}", config.port);
    axum::serve(listener, app).await.unwrap();
}
