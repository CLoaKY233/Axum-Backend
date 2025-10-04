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
        debug!("Starting database health check");
        let (status, message) =
            match timeout(Duration::from_secs(5), self.db.query("RETURN true;")).await {
                Ok(Ok(_)) => {
                    let elapsed = start.elapsed();
                    debug!("Database health check passed in {}ms", elapsed.as_millis());
                    (
                        HealthStatus::Healthy,
                        Some(format!("Response time: {}ms", elapsed.as_millis())),
                    )
                }
                Ok(Err(e)) => {
                    warn!("Database health check failed: {}", e);
                    (HealthStatus::Unhealthy, Some(format!("Query error: {}", e)))
                }
                Err(_) => {
                    warn!("Database health check timed out");
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
