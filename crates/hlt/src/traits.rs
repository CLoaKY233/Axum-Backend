//! # Health Check Trait
//!
//! Defines the `HealthCheck` trait, which is the core of the health check
//! framework.

use crate::ComponentHealth;
use std::time::Duration;

/// A trait for implementing asynchronous health checks.
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check.
    async fn check(&self) -> ComponentHealth;

    /// Returns the timeout for the health check.
    fn timeout(&self) -> Duration;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockHealthy;

    #[async_trait::async_trait]
    impl HealthCheck for MockHealthy {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth::healthy("Mock")
        }

        fn timeout(&self) -> Duration {
            Duration::from_secs(5)
        }
    }

    struct MockUnhealthy;

    #[async_trait::async_trait]
    impl HealthCheck for MockUnhealthy {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth::unhealthy("Mock", "Test failure")
        }

        fn timeout(&self) -> Duration {
            Duration::from_secs(3)
        }
    }

    #[tokio::test]
    async fn test_health_check_trait_healthy() {
        let checker = MockHealthy;
        let result = checker.check().await;

        assert_eq!(result.name, "Mock");
        assert_eq!(result.status, crate::HealthStatus::Healthy);
        assert_eq!(checker.timeout(), Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_health_check_trait_unhealthy() {
        let checker = MockUnhealthy;
        let result = checker.check().await;

        assert_eq!(result.name, "Mock");
        assert_eq!(result.status, crate::HealthStatus::Unhealthy);
        assert_eq!(result.message, Some("Test failure".to_string()));
        assert_eq!(checker.timeout(), Duration::from_secs(3));
    }

    #[tokio::test]
    async fn test_health_check_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn HealthCheck>>();
    }
}
