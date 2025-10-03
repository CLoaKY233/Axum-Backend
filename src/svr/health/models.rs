use serde::Serialize;
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

// Single ComponentHealth struct (removing duplicates)
#[derive(Serialize, Debug)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct SystemHealthResponse {
    pub status: HealthStatus,
    pub components: Vec<ComponentHealth>, // Changed from HashMap to Vec
    pub timestamp: i64,
}

#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check and returns component health status
    async fn check(&self) -> ComponentHealth;
}
