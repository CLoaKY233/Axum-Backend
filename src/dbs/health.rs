use crate::dbs::Database;
use hlt::{ComponentHealth, HealthCheck, HealthStatus};
use tokio::time::{Duration, Instant};
use tracing::{debug, warn};

#[async_trait::async_trait]
impl HealthCheck for Database {
    /// Returns the name of this health check component.
    fn name(&self) -> &'static str {
        "Database"
    }

    // We cast u128 to u64 here because health check latencies will never exceed
    // u64::MAX milliseconds (~584 million years). This is a safe truncation.
    #[allow(clippy::cast_possible_truncation)]
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        debug!("Performing database health check");

        match self.db.query("RETURN true;").await {
            Ok(_) => {
                let elapsed = start.elapsed().as_millis() as u64;
                debug!(latency_ms = elapsed, "Database health check successful");

                // Using builder pattern for cleaner code
                ComponentHealth::builder("Database")
                    .status(HealthStatus::Healthy)
                    .latency_ms(elapsed)
                    .build()
            }
            Err(e) => {
                let elapsed = start.elapsed().as_millis() as u64;
                warn!(error = %e, latency_ms = elapsed, "Database health check failed");

                // Builder pattern makes error cases cleaner
                ComponentHealth::builder("Database")
                    .status(HealthStatus::Unhealthy)
                    .message(format!("Query error: {e}"))
                    .latency_ms(elapsed)
                    .build()
            }
        }
    }

    fn timeout(&self) -> Duration {
        let timeout_secs = env::get_parsed_or_default("DB_HEALTH_CHECK_TIMEOUT", 5);
        Duration::from_secs(timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock test to verify trait implementation compiles
    #[test]
    fn test_database_health_check_name() {
        // This test ensures the Database type implements HealthCheck correctly
        // We can't easily test without a real database connection, but we can
        // verify the trait is implemented properly at compile time
        fn assert_implements_health_check<T: HealthCheck>() {}
        assert_implements_health_check::<Database>();
    }
}
