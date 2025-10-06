use super::error::DatabaseError;
use super::models::{DbConfig, DbConnection};
use std::sync::Arc;
use surrealdb::opt::auth::Namespace;

/// Establishes a connection to the `SurrealDB` database.
/// # Errors
/// Returns `DatabaseError::ConnectionError` or `DatabaseError::AuthenticationError` on failure.
pub async fn establish_connection(config: &DbConfig) -> Result<DbConnection, DatabaseError> {
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
            endpoint: std::env::var("DB_ENDPOINT")
                .map_err(|_| DatabaseError::ConfigError("DB_ENDPOINT not set".to_string()))?,

            namespace: std::env::var("DB_NAMESPACE")
                .map_err(|_| DatabaseError::ConfigError("DB_NAMESPACE not set".to_string()))?,

            database: std::env::var("DB_NAME")
                .map_err(|_| DatabaseError::ConfigError("DB_NAME not set".to_string()))?,

            username: std::env::var("DB_USERNAME")
                .map_err(|_| DatabaseError::ConfigError("DB_USERNAME not set".to_string()))?,

            password: std::env::var("DB_PASSWORD")
                .map_err(|_| DatabaseError::ConfigError("DB_PASSWORD not set".to_string()))?,
        })
    }
    /// Establishes a database connection using this configuration.
    /// # Errors
    /// Returns `DatabaseError` if connection establishment fails.
    pub async fn connect(&self) -> Result<DbConnection, DatabaseError> {
        establish_connection(self).await
    }
}
