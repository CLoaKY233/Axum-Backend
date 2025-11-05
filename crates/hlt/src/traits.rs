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
///     fn name(&self) -> &'static str {  // ✅ Add this
///         "MyService"
///     }
///
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
    /// Returns the name of this health check component.
    ///
    /// This name will be used to identify the component in health reports,
    /// especially when timeouts occur.
    fn name(&self) -> &str;

    /// Performs the health check and returns the component's health status.
    async fn check(&self) -> ComponentHealth;

    /// Returns the timeout duration for this health check.
    fn timeout(&self) -> Duration;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockHealthy;

    #[async_trait::async_trait]
    impl HealthCheck for MockHealthy {
        fn name(&self) -> &'static str {
            "Mock"
        }

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
        fn name(&self) -> &'static str {
            "Mock"
        }

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
        assert_eq!(checker.name(), "Mock");
        let result = checker.check().await;
        assert_eq!(result.name, "Mock");
        assert_eq!(result.status, crate::HealthStatus::Healthy);
        assert_eq!(checker.timeout(), Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_health_check_trait_unhealthy() {
        let checker = MockUnhealthy;
        assert_eq!(checker.name(), "Mock");
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
