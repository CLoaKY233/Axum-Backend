use crate::{ComponentHealth, HealthCheck, SystemHealthResponse};
use futures::future::join_all;
use tracing::{debug, instrument};

/// Registry for managing and executing health checks.
///
/// The registry maintains a collection of health checkers and provides
/// methods to execute them concurrently and aggregate their results.
#[derive(Default)]
pub struct HealthRegistry {
    checkers: Vec<Box<dyn HealthCheck>>,
}

impl HealthRegistry {
    /// Creates a new empty health registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
        }
    }

    /// Registers a new health checker.
    pub fn register(&mut self, checker: Box<dyn HealthCheck>) {
        self.checkers.push(checker);
    }

    /// Executes all registered health checks concurrently.
    ///
    /// Returns an aggregated system health response containing all component
    /// statuses and the overall system health.
    #[instrument(name = "health_check_all", skip(self))]
    pub async fn check_all(&self) -> SystemHealthResponse {
        debug!(
            checker_count = self.checkers.len(),
            "Executing health checks"
        );

        let check_futures = self.checkers.iter().map(|checker| checker.check());
        let components: Vec<ComponentHealth> = join_all(check_futures).await;

        debug!(
            component_count = components.len(),
            "Health checks completed"
        );

        SystemHealthResponse::new(components)
    }

    /// Returns the number of registered health checkers.
    #[must_use]
    pub fn count(&self) -> usize {
        self.checkers.len()
    }

    /// Checks if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checkers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HealthStatus;

    struct MockHealthChecker {
        name: String,
        status: HealthStatus,
    }

    #[async_trait::async_trait]
    impl HealthCheck for MockHealthChecker {
        async fn check(&self) -> ComponentHealth {
            ComponentHealth {
                name: self.name.clone(),
                status: self.status.clone(),
                message: None,
            }
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = HealthRegistry::new();
        assert_eq!(registry.count(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_register() {
        let mut registry = HealthRegistry::new();

        registry.register(Box::new(MockHealthChecker {
            name: "Test1".to_string(),
            status: HealthStatus::Healthy,
        }));

        assert_eq!(registry.count(), 1);
        assert!(!registry.is_empty());

        registry.register(Box::new(MockHealthChecker {
            name: "Test2".to_string(),
            status: HealthStatus::Degraded,
        }));

        assert_eq!(registry.count(), 2);
    }

    #[tokio::test]
    async fn test_registry_check_all_empty() {
        let registry = HealthRegistry::new();
        let response = registry.check_all().await;

        assert_eq!(response.status, HealthStatus::Healthy);
        assert_eq!(response.components.len(), 0);
    }

    #[tokio::test]
    async fn test_registry_check_all_healthy() {
        let mut registry = HealthRegistry::new();

        registry.register(Box::new(MockHealthChecker {
            name: "DB".to_string(),
            status: HealthStatus::Healthy,
        }));

        registry.register(Box::new(MockHealthChecker {
            name: "Cache".to_string(),
            status: HealthStatus::Healthy,
        }));

        let response = registry.check_all().await;

        assert_eq!(response.status, HealthStatus::Healthy);
        assert_eq!(response.components.len(), 2);
    }

    #[tokio::test]
    async fn test_registry_check_all_degraded() {
        let mut registry = HealthRegistry::new();

        registry.register(Box::new(MockHealthChecker {
            name: "DB".to_string(),
            status: HealthStatus::Healthy,
        }));

        registry.register(Box::new(MockHealthChecker {
            name: "Cache".to_string(),
            status: HealthStatus::Degraded,
        }));

        let response = registry.check_all().await;

        assert_eq!(response.status, HealthStatus::Degraded);
        assert_eq!(response.components.len(), 2);
    }

    #[tokio::test]
    async fn test_registry_check_all_unhealthy() {
        let mut registry = HealthRegistry::new();

        registry.register(Box::new(MockHealthChecker {
            name: "DB".to_string(),
            status: HealthStatus::Unhealthy,
        }));

        registry.register(Box::new(MockHealthChecker {
            name: "Cache".to_string(),
            status: HealthStatus::Healthy,
        }));

        let response = registry.check_all().await;

        assert_eq!(response.status, HealthStatus::Unhealthy);
        assert_eq!(response.components.len(), 2);
    }
}
