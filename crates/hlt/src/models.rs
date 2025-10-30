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
    /// Optional latency in milliseconds for the health check.
    /// Typical ranges:
    /// - < 50ms: Excellent
    /// - 50-100ms: Good
    /// - 100-200ms: Acceptable
    /// - > 200ms: Should trigger degraded status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

impl ComponentHealth {
    /// Creates a healthy component status with optional message and latency.
    #[must_use]
    pub fn healthy(
        name: impl Into<String>,
        message: Option<impl Into<String>>,
        latency_ms: Option<u64>,
    ) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            message: message.map(Into::into),
            latency_ms,
        }
    }

    /// Creates a degraded component status with message and optional latency.
    #[must_use]
    pub fn degraded(
        name: impl Into<String>,
        message: impl Into<String>,
        latency_ms: Option<u64>,
    ) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Degraded,
            message: Some(message.into()),
            latency_ms,
        }
    }

    /// Creates an unhealthy component status with message and optional latency.
    #[must_use]
    pub fn unhealthy(
        name: impl Into<String>,
        message: impl Into<String>,
        latency_ms: Option<u64>,
    ) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Unhealthy,
            message: Some(message.into()),
            latency_ms,
        }
    }

    /// Creates a builder for constructing `ComponentHealth` with a fluent API.
    #[must_use]
    pub fn builder(name: impl Into<String>) -> ComponentHealthBuilder {
        ComponentHealthBuilder::new(name)
    }
}

/// Builder for constructing `ComponentHealth` instances with a fluent API.
#[derive(Debug)]
pub struct ComponentHealthBuilder {
    name: String,
    status: HealthStatus,
    message: Option<String>,
    latency_ms: Option<u64>,
}

impl ComponentHealthBuilder {
    /// Creates a new builder with the given component name.
    /// The default status is `Healthy`
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            message: None,
            latency_ms: None,
        }
    }

    /// Sets the health status of the component.
    #[must_use]
    pub fn status(mut self, status: HealthStatus) -> Self {
        self.status = status;
        self
    }

    // Sets an optional message with more details.
    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the latency in milliseconds for the health check.
    #[must_use]
    pub fn latency_ms(mut self, latency: u64) -> Self {
        self.latency_ms = Some(latency);
        self
    }

    /// Builds and returns the `ComponentHealth` instance.
    #[must_use]
    pub fn build(self) -> ComponentHealth {
        ComponentHealth {
            name: self.name,
            status: self.status,
            message: self.message,
            latency_ms: self.latency_ms,
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
        let healthy = ComponentHealth::healthy("Database", None::<String>, Some(123));
        assert_eq!(healthy.name, "Database");
        assert_eq!(healthy.status, HealthStatus::Healthy);
        assert!(healthy.message.is_none());
        assert_eq!(healthy.latency_ms, Some(123));

        let degraded = ComponentHealth::degraded("Cache", "High latency", Some(321));
        assert_eq!(degraded.name, "Cache");
        assert_eq!(degraded.status, HealthStatus::Degraded);
        assert_eq!(degraded.message, Some("High latency".to_string()));
        assert_eq!(degraded.latency_ms, Some(321));

        let unhealthy = ComponentHealth::unhealthy("API", "Connection failed", None);
        assert_eq!(unhealthy.name, "API");
        assert_eq!(unhealthy.status, HealthStatus::Unhealthy);
        assert_eq!(unhealthy.message, Some("Connection failed".to_string()));
        assert!(unhealthy.latency_ms.is_none());
    }

    #[test]
    fn test_builder_pattern_basic() {
        let health = ComponentHealth::builder("Database")
            .status(HealthStatus::Healthy)
            .latency_ms(50)
            .build();

        assert_eq!(health.name, "Database");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.message.is_none());
        assert_eq!(health.latency_ms, Some(50));
    }

    #[test]
    fn test_builder_pattern_with_message() {
        let health = ComponentHealth::builder("Cache")
            .status(HealthStatus::Degraded)
            .message("High latency detected")
            .latency_ms(250)
            .build();

        assert_eq!(health.name, "Cache");
        assert_eq!(health.status, HealthStatus::Degraded);
        assert_eq!(health.message, Some("High latency detected".to_string()));
        assert_eq!(health.latency_ms, Some(250));
    }

    #[test]
    fn test_builder_pattern_minimal() {
        let health = ComponentHealth::builder("API").build();

        assert_eq!(health.name, "API");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.message.is_none());
        assert!(health.latency_ms.is_none());
    }

    #[test]
    fn test_builder_pattern_unhealthy() {
        let health = ComponentHealth::builder("Database")
            .status(HealthStatus::Unhealthy)
            .message("Connection refused")
            .latency_ms(5000)
            .build();

        assert_eq!(health.name, "Database");
        assert_eq!(health.status, HealthStatus::Unhealthy);
        assert_eq!(health.message, Some("Connection refused".to_string()));
        assert_eq!(health.latency_ms, Some(5000));
    }

    #[test]
    fn test_system_health_aggregation_all_healthy() {
        let components = vec![
            ComponentHealth::healthy("DB", None::<String>, None),
            ComponentHealth::healthy("Cache", Some("All good"), Some(50)),
        ];

        let response = SystemHealthResponse::new(components);
        assert_eq!(response.status, HealthStatus::Healthy);
        assert_eq!(response.components.len(), 2);
    }

    #[test]
    fn test_system_health_aggregation_with_degraded() {
        let components = vec![
            ComponentHealth::healthy("DB", None::<String>, None),
            ComponentHealth::degraded("Cache", "Slow", None),
        ];

        let response = SystemHealthResponse::new(components);
        assert_eq!(response.status, HealthStatus::Degraded);
    }

    #[test]
    fn test_system_health_aggregation_with_unhealthy() {
        let components = vec![
            ComponentHealth::healthy("DB", None::<String>, None),
            ComponentHealth::degraded("Cache", "Slow", None),
            ComponentHealth::unhealthy("API", "Down", None),
        ];

        let response = SystemHealthResponse::new(components);
        assert_eq!(response.status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_system_health_timestamp() {
        let components = vec![ComponentHealth::healthy("DB", None::<String>, None)];
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

    #[test]
    fn test_component_serialization_with_latency_and_message() {
        let comp = ComponentHealth::healthy("DB", Some("OK".to_string()), Some(100));
        let json = serde_json::to_string(&comp).unwrap();
        assert!(json.contains(r#""message":"OK""#));
        assert!(json.contains(r#""latency_ms":100"#));

        let comp = ComponentHealth::degraded("Cache", "Slow cache", None);
        let json = serde_json::to_string(&comp).unwrap();
        assert!(json.contains(r#""message":"Slow cache""#));
        assert!(!json.contains("latency_ms"));

        let comp = ComponentHealth::unhealthy("API", "Down", Some(500));
        let json = serde_json::to_string(&comp).unwrap();
        assert!(json.contains(r#""latency_ms":500"#));
        assert!(json.contains(r#""message":"Down""#));
    }
}
