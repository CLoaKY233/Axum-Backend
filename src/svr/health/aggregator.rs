use crate::{
    dbs::models::Database,
    svr::{config::state::AppState, health::models::*},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use futures::future::join_all;
use std::sync::Arc;

pub async fn aggregate_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let components: Vec<Box<dyn HealthCheck>> = vec![Box::new(Database {
        db: state.db_connection.clone(),
    })];

    let check_futures = components.iter().map(|checker| checker.check());

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

    let response = SystemHealthResponse {
        status: overall_status.clone(),
        components: results,
        timestamp: chrono::Utc::now().timestamp(),
    };

    let http_status = match overall_status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK, // Still serving traffic
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (http_status, Json(response))
}
