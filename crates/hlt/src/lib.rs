//! # Health Check Framework
//!
//! A lightweight and extensible health check framework for Axum applications.

mod models;
mod registry;
mod traits;

// Public API exports (NO handler export!)
pub use models::{ComponentHealth, ComponentHealthBuilder, HealthStatus, SystemHealthResponse};
pub use registry::HealthRegistry;
pub use traits::HealthCheck;
