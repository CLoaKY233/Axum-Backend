use crate::sys::{
    config::state::AppState,
    health::models::{ComponentHealth, HealthStatus, SystemHealthResponse},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use futures::future::join_all;
use std::sync::Arc;

pub async fn aggregate_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let check_futures = state.health_checkers.iter().map(|checker| checker.check());

    let results: Vec<ComponentHealth> = join_all(check_futures).await;

    let overall_status = if results
        .iter()
        .any(|r| matches!(r.status, HealthStatus::Unhealthy))
    {
        HealthStatus::Unhealthy
    } else if results
        .iter()
        .any(|r| matches!(r.status, HealthStatus::Degraded))
    {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    };

    let http_status = match overall_status {
        HealthStatus::Healthy | HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };
    let response = SystemHealthResponse {
        status: overall_status,
        components: results,
        timestamp: chrono::Utc::now().timestamp(),
    };

    (http_status, Json(response))
}
