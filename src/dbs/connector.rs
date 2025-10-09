use super::error::DatabaseError;
use super::models::{DbConfig, DbConnection};
use crate::sys::env;
use std::sync::Arc;
use surrealdb::opt::auth::Namespace;

/// Establishes a connection to the `SurrealDB` database.
/// # Errors
/// Returns `DatabaseError::ConnectionError` or `DatabaseError::AuthenticationError` on failure.
pub async fn connect(config: &DbConfig) -> Result<DbConnection, DatabaseError> {
    let db = surrealdb::engine::any::connect(&config.endpoint)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    db.use_ns(&config.namespace)
        .use_db(&config.database)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    db.signin(Namespace {
        namespace: &config.namespace,
        username: &config.username,
        password: &config.password,
    })
    .await
    .map_err(|e| DatabaseError::AuthenticationError(e.to_string()))?;

    Ok(Arc::new(db))
}

impl DbConfig {
    /// Creates a database configuration from environment variables.
    /// # Errors
    /// Returns `DatabaseError::ConfigError` if any required environment variable is not set.
    pub fn from_env() -> Result<Self, DatabaseError> {
        Ok(Self {
            endpoint: env::get_required("DB_ENDPOINT")
                .map_err(|e| DatabaseError::ConfigError(e.to_string()))?,

            namespace: env::get_required("DB_NAMESPACE")
                .map_err(|e| DatabaseError::ConfigError(e.to_string()))?,

            database: env::get_required("DB_NAME")
                .map_err(|e| DatabaseError::ConfigError(e.to_string()))?,

            username: env::get_required("DB_USERNAME")
                .map_err(|e| DatabaseError::ConfigError(e.to_string()))?,

            password: env::get_required("DB_PASSWORD")
                .map_err(|e| DatabaseError::ConfigError(e.to_string()))?,
        })
    }
}
