//! # Health Check Trait
//!
//! Defines the `HealthCheck` trait, which is the core of the health check
//! framework.

use crate::ComponentHealth;
use std::time::Duration;

/// A trait for implementing asynchronous health checks.
///
/// # Example
///
/// ```
/// use hlt::{ComponentHealth, HealthCheck, HealthStatus};
/// use tokio::time::Duration;
///
/// struct MyServiceChecker;
///
/// #[async_trait::async_trait]
/// impl HealthCheck for MyServiceChecker {
///     async fn check(&self) -> ComponentHealth {
///         // Your health check logic here
///         ComponentHealth::builder("MyService")
///             .status(HealthStatus::Healthy)
///             .build()
///     }
///
///     fn timeout(&self) -> Duration {
///         Duration::from_secs(3)
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check and returns the component's health status.
    ///
    /// This method should perform the actual health verification logic
    /// (e.g., database ping, HTTP request, file check) and return a
    /// `ComponentHealth` instance with status, optional message, and latency.
    async fn check(&self) -> ComponentHealth;

    /// Returns the timeout duration for this health check.
    ///
    /// If the health check takes longer than this duration, it will be
    /// cancelled and reported as unhealthy by the registry.
    fn timeout(&self) -> Duration;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockHealthy;

    #[async_trait::async_trait]
    impl HealthCheck for MockHealthy {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth::healthy("Mock", None::<String>, None)
        }

        fn timeout(&self) -> Duration {
            Duration::from_secs(5)
        }
    }

    struct MockUnhealthy;

    #[async_trait::async_trait]
    impl HealthCheck for MockUnhealthy {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth::unhealthy("Mock", "Test failure", None)
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
