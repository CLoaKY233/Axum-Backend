use super::models::ComponentHealth;

#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check and returns component health status.
    async fn check(&self) -> ComponentHealth;
}
