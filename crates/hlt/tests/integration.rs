use hlt::{ComponentHealth, HealthCheck, HealthRegistry, HealthStatus};
use std::sync::Arc;
use tokio::time::{Duration, sleep};

struct DatabaseHealth {
    should_fail: bool,
}

#[async_trait::async_trait]
impl HealthCheck for DatabaseHealth {
    fn name(&self) -> &'static str {
        "Database"
    }

    async fn check(&self) -> ComponentHealth {
        sleep(Duration::from_millis(10)).await;
        if self.should_fail {
            ComponentHealth::unhealthy("Database", "Connection timeout", None)
        } else {
            ComponentHealth::healthy("Database", None::<String>, None)
        }
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(5)
    }
}

struct CacheHealth {
    latency_ms: u64,
}

#[async_trait::async_trait]
impl HealthCheck for CacheHealth {
    fn name(&self) -> &'static str {
        "Cache"
    }

    async fn check(&self) -> ComponentHealth {
        sleep(Duration::from_millis(5)).await;
        if self.latency_ms > 100 {
            ComponentHealth::degraded(
                "Cache",
                format!("High latency: {}ms", self.latency_ms),
                Some(self.latency_ms),
            )
        } else {
            ComponentHealth::healthy("Cache", None::<String>, Some(self.latency_ms))
        }
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(3)
    }
}

#[tokio::test]
async fn test_full_health_check_workflow() {
    let mut registry = HealthRegistry::new();
    registry.register(Box::new(DatabaseHealth { should_fail: false }));
    registry.register(Box::new(CacheHealth { latency_ms: 50 }));

    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Healthy);
    assert_eq!(response.components.len(), 2);

    let names: Vec<&str> = response
        .components
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    assert!(names.contains(&"Database"));
    assert!(names.contains(&"Cache"));
}

#[tokio::test]
async fn test_degraded_system_status() {
    let mut registry = HealthRegistry::new();
    registry.register(Box::new(DatabaseHealth { should_fail: false }));
    registry.register(Box::new(CacheHealth { latency_ms: 150 }));

    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Degraded);
    assert_eq!(response.components.len(), 2);

    let cache = response
        .components
        .iter()
        .find(|c| c.name == "Cache")
        .unwrap();
    assert_eq!(cache.status, HealthStatus::Degraded);
    assert!(cache.message.is_some());
}

#[tokio::test]
async fn test_unhealthy_system_status() {
    let mut registry = HealthRegistry::new();
    registry.register(Box::new(DatabaseHealth { should_fail: true }));
    registry.register(Box::new(CacheHealth { latency_ms: 50 }));

    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Unhealthy);
    assert_eq!(response.components.len(), 2);

    let db = response
        .components
        .iter()
        .find(|c| c.name == "Database")
        .unwrap();
    assert_eq!(db.status, HealthStatus::Unhealthy);
    assert_eq!(db.message, Some("Connection timeout".to_string()));
}

#[tokio::test]
async fn test_concurrent_health_checks() {
    let mut registry = HealthRegistry::new();

    for _ in 0..10 {
        registry.register(Box::new(DatabaseHealth { should_fail: false }));
    }

    let start = std::time::Instant::now();
    let response = registry.check_all().await;
    let duration = start.elapsed();

    // Increased threshold for CI stability
    assert!(duration < Duration::from_millis(150));
    assert_eq!(response.components.len(), 10);
    assert_eq!(response.status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_mixed_component_statuses() {
    let mut registry = HealthRegistry::new();
    registry.register(Box::new(DatabaseHealth { should_fail: true }));
    registry.register(Box::new(CacheHealth { latency_ms: 150 }));

    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Unhealthy);
    assert_eq!(response.components.len(), 2);
}

#[tokio::test]
async fn test_empty_registry() {
    let registry = HealthRegistry::new();
    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Healthy);
    assert_eq!(response.components.len(), 0);
}

#[tokio::test]
async fn test_response_serialization() {
    let mut registry = HealthRegistry::new();
    registry.register(Box::new(DatabaseHealth { should_fail: false }));

    let response = registry.check_all().await;
    let json = serde_json::to_string(&response).unwrap();

    assert!(json.contains("\"status\":\"healthy\""));
    assert!(json.contains("\"components\""));
    assert!(json.contains("\"timestamp\""));
}

#[tokio::test]
async fn test_thread_safety() {
    let registry = Arc::new({
        let mut reg = HealthRegistry::new();
        reg.register(Box::new(DatabaseHealth { should_fail: false }));
        reg
    });

    let mut handles = vec![];

    for _ in 0..5 {
        let reg = Arc::clone(&registry);
        let handle = tokio::spawn(async move {
            let response = reg.check_all().await;
            assert_eq!(response.status, HealthStatus::Healthy);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_timeout_enforcement() {
    struct TimeoutChecker {
        delay: Duration,
        timeout_duration: Duration,
    }

    #[async_trait::async_trait]
    impl HealthCheck for TimeoutChecker {
        fn name(&self) -> &'static str {
            "TimeoutTest"
        }

        async fn check(&self) -> ComponentHealth {
            sleep(self.delay).await;
            ComponentHealth::healthy("TimeoutTest", None::<String>, None)
        }

        fn timeout(&self) -> Duration {
            self.timeout_duration
        }
    }

    let mut registry = HealthRegistry::new();
    registry.register(Box::new(TimeoutChecker {
        delay: Duration::from_secs(2),
        timeout_duration: Duration::from_millis(100),
    }));

    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Unhealthy);
    assert_eq!(response.components.len(), 1);

    let component = &response.components[0];
    assert_eq!(component.name, "TimeoutTest"); // Correctly identified!
    assert_eq!(component.status, HealthStatus::Unhealthy);
    assert!(component.message.as_ref().unwrap().contains("timed out"));
}

#[tokio::test]
async fn test_different_timeouts_per_component() {
    struct FastChecker;

    #[async_trait::async_trait]
    impl HealthCheck for FastChecker {
        fn name(&self) -> &'static str {
            "Fast"
        }

        async fn check(&self) -> ComponentHealth {
            sleep(Duration::from_millis(10)).await;
            ComponentHealth::healthy("Fast", None::<String>, None)
        }

        fn timeout(&self) -> Duration {
            Duration::from_millis(50)
        }
    }

    struct SlowChecker;

    #[async_trait::async_trait]
    impl HealthCheck for SlowChecker {
        fn name(&self) -> &'static str {
            "Slow"
        }

        async fn check(&self) -> ComponentHealth {
            sleep(Duration::from_millis(100)).await;
            ComponentHealth::healthy("Slow", None::<String>, None)
        }

        fn timeout(&self) -> Duration {
            Duration::from_secs(1)
        }
    }

    let mut registry = HealthRegistry::new();
    registry.register(Box::new(FastChecker));
    registry.register(Box::new(SlowChecker));

    let response = registry.check_all().await;

    assert_eq!(response.status, HealthStatus::Healthy);
    assert_eq!(response.components.len(), 2);
}

#[tokio::test]
async fn test_builder_pattern_in_health_check() {
    struct BuilderStyleChecker {
        latency: u64,
    }

    #[async_trait::async_trait]
    impl HealthCheck for BuilderStyleChecker {
        fn name(&self) -> &'static str {
            "BuilderTest"
        }

        async fn check(&self) -> ComponentHealth {
            sleep(Duration::from_millis(5)).await;

            let mut builder = ComponentHealth::builder("BuilderTest").latency_ms(self.latency);

            if self.latency < 100 {
                builder = builder.status(HealthStatus::Healthy);
            } else if self.latency < 200 {
                builder = builder
                    .status(HealthStatus::Degraded)
                    .message(format!("Moderate latency: {}ms", self.latency));
            } else {
                builder = builder
                    .status(HealthStatus::Unhealthy)
                    .message(format!("High latency: {}ms", self.latency));
            }

            builder.build()
        }

        fn timeout(&self) -> Duration {
            Duration::from_secs(5)
        }
    }

    let mut registry = HealthRegistry::new();
    registry.register(Box::new(BuilderStyleChecker { latency: 50 }));
    let response = registry.check_all().await;
    assert_eq!(response.status, HealthStatus::Healthy);

    let mut registry = HealthRegistry::new();
    registry.register(Box::new(BuilderStyleChecker { latency: 150 }));
    let response = registry.check_all().await;
    assert_eq!(response.status, HealthStatus::Degraded);

    let mut registry = HealthRegistry::new();
    registry.register(Box::new(BuilderStyleChecker { latency: 250 }));
    let response = registry.check_all().await;
    assert_eq!(response.status, HealthStatus::Unhealthy);
}
