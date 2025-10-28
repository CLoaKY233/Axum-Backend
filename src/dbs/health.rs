use crate::dbs::Database;
use hlt::{ComponentHealth, HealthCheck};
use tokio::time::{Duration, Instant, timeout};
use tracing::{debug, warn};

#[async_trait::async_trait]
impl HealthCheck for Database {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        debug!("Performing database health check");

        let timeout_secs = env::get_parsed_or_default("DB_HEALTH_CHECK_TIMEOUT", 5);

        match timeout(
            Duration::from_secs(timeout_secs),
            self.db.query("RETURN true;"),
        )
        .await
        {
            Ok(Ok(_)) => {
                let elapsed = start.elapsed();
                debug!(
                    latency_ms = elapsed.as_millis(),
                    "Database health check successful"
                );
                ComponentHealth::healthy("Database")
            }
            Ok(Err(e)) => {
                warn!(error = %e, "Database health check failed");
                ComponentHealth::unhealthy("Database", format!("Query error: {e}"))
            }
            Err(_) => {
                warn!(
                    timeout_secs = timeout_secs,
                    "Database health check timed out"
                );
                ComponentHealth::unhealthy(
                    "Database",
                    format!("Health check timeout after {timeout_secs} seconds"),
                )
            }
        }
    }
}
