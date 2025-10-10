use super::models::Database;
use crate::sys::{
    env,
    health::models::HealthCheck,
    health::models::{ComponentHealth, HealthStatus},
};
use tokio::time::{Duration, Instant, timeout};
use tracing::{debug, warn};

#[async_trait::async_trait]
impl HealthCheck for Database {
    /// Performs a health check on the database.
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        debug!("Performing database health check");
        let timeout_secs = env::get_parsed_or_default("DB_HEALTH_CHECK_TIMEOUT", 5);
        let (status, message) = match timeout(
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
                (
                    HealthStatus::Healthy,
                    Some(format!("Response time: {}ms", elapsed.as_millis())),
                )
            }
            Ok(Err(e)) => {
                warn!(error = %e, "Database health check failed");
                (HealthStatus::Unhealthy, Some(format!("Query error: {e}")))
            }
            Err(_) => {
                warn!(
                    timeout_secs = timeout_secs,
                    "Database health check timed out"
                );
                (
                    HealthStatus::Unhealthy,
                    Some(format!("Health check timeout after {timeout_secs} seconds")),
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
