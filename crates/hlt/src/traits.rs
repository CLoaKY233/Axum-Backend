use crate::ComponentHealth;

/// Trait for implementing health checks on system components.
///
/// Implement this trait for any component that needs health monitoring.
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check and returns the component's health status.
    ///
    /// This method should be non-blocking and complete quickly.
    async fn check(&self) -> ComponentHealth;
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
    }

    struct MockUnhealthy;

    #[async_trait::async_trait]
    impl HealthCheck for MockUnhealthy {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth::unhealthy("Mock", "Test failure")
        }
    }

    #[tokio::test]
    async fn test_health_check_trait_healthy() {
        let checker = MockHealthy;
        let result = checker.check().await;

        assert_eq!(result.name, "Mock");
        assert_eq!(result.status, crate::HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_health_check_trait_unhealthy() {
        let checker = MockUnhealthy;
        let result = checker.check().await;

        assert_eq!(result.name, "Mock");
        assert_eq!(result.status, crate::HealthStatus::Unhealthy);
        assert_eq!(result.message, Some("Test failure".to_string()));
    }

    #[tokio::test]
    async fn test_health_check_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn HealthCheck>>();
    }
}
