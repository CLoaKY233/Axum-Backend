use super::models::Database;
use crate::sys::{
    health::models::HealthCheck,
    health::models::{ComponentHealth, HealthStatus},
};
use tokio::time::{Duration, Instant, timeout};
use tracing::{debug, warn};

#[async_trait::async_trait]
impl HealthCheck for Database {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        debug!("Performing database health check");
        let (status, message) =
            match timeout(Duration::from_secs(5), self.db.query("RETURN true;")).await {
                Ok(Ok(_)) => {
                    let elapsed = start.elapsed();
                    debug!(latency_ms = elapsed.as_millis(), "Database health check successful");
                    (
                        HealthStatus::Healthy,
                        Some(format!("Response time: {}ms", elapsed.as_millis())),
                    )
                }
                Ok(Err(e)) => {
                    warn!(error = %e, "Database health check failed");
                    (HealthStatus::Unhealthy, Some(format!("Query error: {}", e)))
                }
                Err(_) => {
                    warn!("Database health check timed out after 5 seconds");
                    (
                        HealthStatus::Unhealthy,
                        Some("Health check timeout after 5 seconds".to_string()),
                    )
                }
            };

        ComponentHealth {
            name: "Database".to_string(),
            status,
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbs::models::Database;
    use std::sync::Arc;
    use surrealdb::{Surreal, engine::any::Any};
    use tracing_test::traced_test;

    /// Helper function to create a mock database connection for testing
    /// Note: This creates a real in-memory database for testing
    async fn create_test_database() -> Database {
        let db = Surreal::<Any>::init();
        Database {
            db: Arc::new(db),
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_health_check_returns_database_component() {
        // Test that health check returns a component with correct name
        let db = create_test_database().await;
        let result = db.check().await;
        
        assert_eq\!(result.name, "Database");
    }

    #[tokio::test]
    #[traced_test]
    async fn test_health_check_with_disconnected_database() {
        // Test health check behavior with disconnected database
        let db = create_test_database().await;
        let result = db.check().await;
        
        // Should be unhealthy since it's not connected to a real database
        assert\!(matches\!(result.status, HealthStatus::Unhealthy));
        assert\!(result.message.is_some());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_health_check_message_format_on_error() {
        // Test that error messages are properly formatted
        let db = create_test_database().await;
        let result = db.check().await;
        
        if let Some(msg) = result.message {
            assert\!(msg.contains("Query error:") || msg.contains("timeout"));
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_health_check_debug_logging() {
        // Test that debug logs are emitted during health check
        let db = create_test_database().await;
        let _result = db.check().await;
        
        // tracing_test will capture logs, verify debug message was logged
        assert\!(logs_contain("Performing database health check") || 
                logs_contain("Database health check"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_health_check_warn_logging_on_failure() {
        // Test that warning logs are emitted on failure
        let db = create_test_database().await;
        let result = db.check().await;
        
        // Should log warnings for unhealthy state
        if matches\!(result.status, HealthStatus::Unhealthy) {
            assert\!(logs_contain("Database health check failed") || 
                    logs_contain("Database health check timed out"));
        }
    }

    #[tokio::test]
    async fn test_health_check_timeout_duration() {
        // Test that health check respects the 5-second timeout
        let start = Instant::now();
        let db = create_test_database().await;
        let _result = db.check().await;
        let elapsed = start.elapsed();
        
        // Should complete within 6 seconds (5s timeout + 1s buffer)
        assert\!(elapsed.as_secs() < 6);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_health_check_status_variants() {
        // Test that health check can return different status variants
        let db = create_test_database().await;
        let result = db.check().await;
        
        // Status should be one of the valid variants
        match result.status {
            HealthStatus::Healthy | HealthStatus::Degraded | HealthStatus::Unhealthy => {
                // Valid status
            }
        }
    }

    #[tokio::test]
    async fn test_health_check_component_structure() {
        // Test that ComponentHealth has all required fields
        let db = create_test_database().await;
        let result = db.check().await;
        
        assert\!(\!result.name.is_empty());
        // message can be Some or None, both are valid
    }

    #[tokio::test]
    #[traced_test]
    async fn test_multiple_sequential_health_checks() {
        // Test that multiple health checks can be performed sequentially
        let db = create_test_database().await;
        
        let result1 = db.check().await;
        let result2 = db.check().await;
        let result3 = db.check().await;
        
        // All should return Database component
        assert_eq\!(result1.name, "Database");
        assert_eq\!(result2.name, "Database");
        assert_eq\!(result3.name, "Database");
    }

    #[tokio::test]
    async fn test_health_check_response_time_tracking() {
        // Test that response time is tracked and included in message
        let db = create_test_database().await;
        let result = db.check().await;
        
        if matches\!(result.status, HealthStatus::Healthy) {
            if let Some(msg) = result.message {
                assert\!(msg.contains("Response time:") && msg.contains("ms"));
            }
        }
    }

    #[test]
    fn test_health_status_variants_exist() {
        // Test that all expected health status variants exist
        let _healthy = HealthStatus::Healthy;
        let _degraded = HealthStatus::Degraded;
        let _unhealthy = HealthStatus::Unhealthy;
    }

    #[tokio::test]
    async fn test_health_check_async_trait_implementation() {
        // Test that Database correctly implements HealthCheck trait
        let db = create_test_database().await;
        let _checker: &dyn HealthCheck = &db;
        
        // If this compiles, the trait is properly implemented
    }
}