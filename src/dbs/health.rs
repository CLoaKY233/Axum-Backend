use crate::dbs::Database;
use hlt::{ComponentHealth, HealthCheck, HealthStatus};
use tokio::time::{Duration, Instant};
use tracing::{debug, warn};

#[async_trait::async_trait]
impl HealthCheck for Database {
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

// ADD MOCK DATABASE TESTS LATER IF NEEDED
