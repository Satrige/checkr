use crate::models::checker::{CheckResult, CheckStatus, Checker};
use axum::{Json, Router, http::StatusCode, routing::get};
use futures::future::join_all;
use std::sync::Arc;
use tokio::task::spawn_blocking;

pub struct HealthRouters {
    checkers: Arc<Vec<Arc<dyn Checker + Send + Sync>>>,
}

impl HealthRouters {
    pub fn new(checkers: Vec<Arc<dyn Checker + Send + Sync>>) -> Self {
        HealthRouters {
            checkers: Arc::new(checkers),
        }
    }

    pub fn get_rountes(&self) -> Router {
        let clonned_checkers = self.checkers.clone();
        Router::new().route("/", get(move || handler(clonned_checkers)))
    }
}

async fn handler(
    checkers: Arc<Vec<Arc<dyn Checker + Send + Sync>>>,
) -> (StatusCode, Json<Vec<CheckResult>>) {
    let handlers = checkers
        .iter() // Iterate over the Arcs
        .cloned() // Bump each reference count (Arc) by 1
        .map(|checker| {
            spawn_blocking(move || {
                let checker_name = checker.get_name();
                let result = checker.check().unwrap_or_else(|e| {
                    tracing::warn!("Error: checker: {} | {}", checker_name, e.to_string());

                    CheckResult::new(
                        checker_name.to_string(),
                        CheckStatus::ERROR,
                        Some(e.to_string()),
                    )
                });

                result
            })
        });

    let check_results: Vec<CheckResult> = join_all(handlers)
        .await
        .into_iter()
        .map(|res| match res {
            Ok(single_result) => single_result,
            Err(e) => {
                tracing::error!("Join error: {}", e.to_string());

                CheckResult::new(
                    "unknown".to_string(),
                    CheckStatus::ERROR,
                    Some("Join error".to_string()),
                )
            }
        })
        .collect();

    (StatusCode::OK, Json(check_results))
}
