use std::sync::Arc;

use axum::Router;

use super::routes::health::HealthRouters;
use crate::{config::AppConfig, domain::Checker};

pub async fn start_server(config: AppConfig, checkers: Vec<Arc<dyn Checker + Send + Sync>>) {
    let app = Router::new().nest("/health", HealthRouters::new(checkers).get_rountes());

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Started checkr on port: {}", config.port);
    axum::serve(listener, app).await.unwrap();
}
