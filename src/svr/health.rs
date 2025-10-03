use crate::{
    dbs::health::check_database_health,
    svr::{models::*, state::AppState},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::{collections::HashMap, sync::Arc};

pub async fn aggregate_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut components = HashMap::new();
    let mut overall_healthy = true;

    // Check database health
    match check_database_health(&state.db_connection).await {
        Ok(msg) => {
            components.insert(
                "database".to_string(),
                ComponentHealth {
                    status: "healthy".to_string(),
                    message: Some(msg),
                },
            );
        }
        Err(e) => {
            overall_healthy = false;
            components.insert(
                "database".to_string(),
                ComponentHealth {
                    status: "unhealthy".to_string(),
                    message: Some(e.to_string()),
                },
            );
        }
    }

    // Add more component checks here as you build them:
    // - Cache health
    // - External API health
    // - Auth service health

    // Determine overall status
    let status = if overall_healthy {
        HealthStatus::Healthy
    } else {
        HealthStatus::Unhealthy
    };

    let response = SystemHealthResponse {
        status,
        components,
        timestamp: chrono::Utc::now().timestamp(),
    };

    // Return appropriate HTTP status code
    let http_status = if overall_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (http_status, Json(response))
}
