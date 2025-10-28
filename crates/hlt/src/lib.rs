//! # hlt - Health Check Framework
//!
//! A lightweight, extensible health check framework with Axum integration.

mod models;
mod registry;
mod traits;

// Public API exports (NO handler export!)
pub use models::{ComponentHealth, HealthStatus, SystemHealthResponse};
pub use registry::HealthRegistry;
pub use traits::HealthCheck;
