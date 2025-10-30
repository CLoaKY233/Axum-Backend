use crate::dbs::Database;
use hlt::{ComponentHealth, HealthCheck};
use tokio::time::{Duration, Instant};
use tracing::{debug, warn};

#[async_trait::async_trait]
impl HealthCheck for Database {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        debug!("Performing database health check");

        match self.db.query("RETURN true;").await {
            Ok(_) => {
                let elapsed = start.elapsed();
                debug!(
                    latency_ms = elapsed.as_millis(),
                    "Database health check successful"
                );
                ComponentHealth::healthy("Database", None::<String>, Some(elapsed.as_millis()))
            }
            Err(e) => {
                let elapsed = start.elapsed();
                warn!(
                    error = %e,
                    latency_ms = elapsed.as_millis(),
                    "Database health check failed"
                );
                ComponentHealth::unhealthy(
                    "Database",
                    format!("Query error: {e}"),
                    Some(elapsed.as_millis()),
                )
            }
        }
    }

    fn timeout(&self) -> Duration {
        // Read timeout from environment, default to 5 seconds
        let timeout_secs = env::get_parsed_or_default("DB_HEALTH_CHECK_TIMEOUT", 5);
        Duration::from_secs(timeout_secs)
    }
}
// ADD MOCK DATABASE TESTS LATER IF NEEDED
