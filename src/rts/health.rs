use crate::sys::config::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use hlt::HealthStatus;
use std::sync::Arc;
use tracing::{debug, warn};

/// Axum handler for health check endpoints.
///
/// Returns the aggregated health status of all registered components.
///
/// # HTTP Status Codes
///
/// - `200 OK` - System is healthy or degraded
/// - `503 Service Unavailable` - System is unhealthy
pub async fn health_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Extract the registry from AppState and call check_all
    let response = state.health_registry.check_all().await;

    let http_status = match response.status {
        HealthStatus::Healthy => {
            debug!("Health check: all components healthy");
            StatusCode::OK
        }
        HealthStatus::Degraded => {
            warn!("Health check: system degraded");
            StatusCode::OK
        }
        HealthStatus::Unhealthy => {
            warn!("Health check: system unhealthy");
            StatusCode::SERVICE_UNAVAILABLE
        }
    };

    (http_status, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hlt::{ComponentHealth, HealthCheck, HealthRegistry};
    use std::time::Duration;

    struct MockChecker {
        status: HealthStatus,
    }

    #[async_trait::async_trait]
    impl HealthCheck for MockChecker {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth::builder("Mock")
                .status(self.status.clone())
                .build()
        }

        fn timeout(&self) -> Duration {
            Duration::from_secs(5)
        }
    }

    #[tokio::test]
    async fn test_health_registry_directly() {
        let mut registry = HealthRegistry::new();
        registry.register(Box::new(MockChecker {
            status: HealthStatus::Healthy,
        }));

        let response = registry.check_all().await;
        assert_eq!(response.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_health_status_mapping() {
        let healthy = HealthStatus::Healthy;
        let status = match healthy {
            HealthStatus::Healthy | HealthStatus::Degraded => StatusCode::OK,
            HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
        };
        assert_eq!(status, StatusCode::OK);

        let unhealthy = HealthStatus::Unhealthy;
        let status = match unhealthy {
            HealthStatus::Healthy | HealthStatus::Degraded => StatusCode::OK,
            HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
        };
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_health_registry_with_multiple_statuses() {
        let mut registry = HealthRegistry::new();

        registry.register(Box::new(MockChecker {
            status: HealthStatus::Healthy,
        }));

        registry.register(Box::new(MockChecker {
            status: HealthStatus::Degraded,
        }));

        let response = registry.check_all().await;
        assert_eq!(response.status, HealthStatus::Degraded);
        assert_eq!(response.components.len(), 2);
    }

    #[tokio::test]
    async fn test_health_registry_with_unhealthy() {
        let mut registry = HealthRegistry::new();

        registry.register(Box::new(MockChecker {
            status: HealthStatus::Unhealthy,
        }));

        let response = registry.check_all().await;
        assert_eq!(response.status, HealthStatus::Unhealthy);
        assert_eq!(response.components.len(), 1);
    }
}
