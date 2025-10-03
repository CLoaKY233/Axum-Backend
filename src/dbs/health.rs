use super::models::Database;
use crate::sys::{
    health::models::HealthCheck,
    health::models::{ComponentHealth, HealthStatus},
};
use tokio::time::{Duration, Instant, timeout};

#[async_trait::async_trait]
impl HealthCheck for Database {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        match timeout(Duration::from_secs(5), self.db.query("RETURN true;")).await {
            Ok(Ok(_)) => {
                let elapsed = start.elapsed();
                ComponentHealth {
                    name: "Database".to_string(),
                    status: HealthStatus::Healthy,
                    message: Some(format!("Response time: {}ms", elapsed.as_millis())),
                }
            }
            Ok(Err(e)) => ComponentHealth {
                name: "Database".to_string(),
                status: HealthStatus::Unhealthy,
                message: Some(format!("Query error: {}", e)),
            },
            Err(_) => ComponentHealth {
                name: "Database".to_string(),
                status: HealthStatus::Unhealthy,
                message: Some("Health check timeout after 5 seconds".to_string()),
            },
        }
    }
}
