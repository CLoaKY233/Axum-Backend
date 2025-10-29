//! # Health Check Models
//!
//! Defines the data structures used for health checking.

use serde::Serialize;

/// The health status of a component.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// The component is healthy.
    Healthy,
    /// The component is in a degraded state.
    Degraded,
    /// The component is unhealthy.
    Unhealthy,
}

/// The health of a single component.
#[derive(Serialize, Debug, Clone)]
pub struct ComponentHealth {
    /// The name of the component.
    pub name: String,
    /// The health status of the component.
    pub status: HealthStatus,
    /// An optional message with more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ComponentHealth {
    /// Creates a healthy component status.
    #[must_use]
    pub fn healthy(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            message: None,
        }
    }

    /// Creates a degraded component status with a message.
    #[must_use]
    pub fn degraded(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Degraded,
            message: Some(message.into()),
        }
    }

    /// Creates an unhealthy component status with a message.
    #[must_use]
    pub fn unhealthy(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Unhealthy,
            message: Some(message.into()),
        }
    }
}

/// The overall health response for the system.
#[derive(Serialize, Debug)]
pub struct SystemHealthResponse {
    /// The overall system health status.
    pub status: HealthStatus,
    /// The health of individual components.
    pub components: Vec<ComponentHealth>,
    /// The timestamp of the health check.
    pub timestamp: i64,
}

impl SystemHealthResponse {
    /// Creates a new system health response.
    #[must_use]
    pub fn new(components: Vec<ComponentHealth>) -> Self {
        let status = Self::aggregate_status(&components);
        let timestamp = chrono::Utc::now().timestamp();

        Self {
            status,
            components,
            timestamp,
        }
    }

    /// Aggregates component statuses into a system-wide status.
    fn aggregate_status(components: &[ComponentHealth]) -> HealthStatus {
        if components
            .iter()
            .any(|c| matches!(c.status, HealthStatus::Unhealthy))
        {
            HealthStatus::Unhealthy
        } else if components
            .iter()
            .any(|c| matches!(c.status, HealthStatus::Degraded))
        {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_health_constructors() {
        let healthy = ComponentHealth::healthy("Database");
        assert_eq!(healthy.name, "Database");
        assert_eq!(healthy.status, HealthStatus::Healthy);
        assert!(healthy.message.is_none());

        let degraded = ComponentHealth::degraded("Cache", "High latency");
        assert_eq!(degraded.name, "Cache");
        assert_eq!(degraded.status, HealthStatus::Degraded);
        assert_eq!(degraded.message, Some("High latency".to_string()));

        let unhealthy = ComponentHealth::unhealthy("API", "Connection failed");
        assert_eq!(unhealthy.name, "API");
        assert_eq!(unhealthy.status, HealthStatus::Unhealthy);
        assert_eq!(unhealthy.message, Some("Connection failed".to_string()));
    }

    #[test]
    fn test_system_health_aggregation_all_healthy() {
        let components = vec![
            ComponentHealth::healthy("DB"),
            ComponentHealth::healthy("Cache"),
        ];

        let response = SystemHealthResponse::new(components);
        assert_eq!(response.status, HealthStatus::Healthy);
        assert_eq!(response.components.len(), 2);
    }

    #[test]
    fn test_system_health_aggregation_with_degraded() {
        let components = vec![
            ComponentHealth::healthy("DB"),
            ComponentHealth::degraded("Cache", "Slow"),
        ];

        let response = SystemHealthResponse::new(components);
        assert_eq!(response.status, HealthStatus::Degraded);
    }

    #[test]
    fn test_system_health_aggregation_with_unhealthy() {
        let components = vec![
            ComponentHealth::healthy("DB"),
            ComponentHealth::degraded("Cache", "Slow"),
            ComponentHealth::unhealthy("API", "Down"),
        ];

        let response = SystemHealthResponse::new(components);
        assert_eq!(response.status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_system_health_timestamp() {
        let components = vec![ComponentHealth::healthy("DB")];
        let response = SystemHealthResponse::new(components);

        let now = chrono::Utc::now().timestamp();
        assert!((response.timestamp - now).abs() <= 1);
    }

    #[test]
    fn test_health_status_serialization() {
        let json = serde_json::to_string(&HealthStatus::Healthy).unwrap();
        assert_eq!(json, "\"healthy\"");

        let json = serde_json::to_string(&HealthStatus::Degraded).unwrap();
        assert_eq!(json, "\"degraded\"");

        let json = serde_json::to_string(&HealthStatus::Unhealthy).unwrap();
        assert_eq!(json, "\"unhealthy\"");
    }
}
