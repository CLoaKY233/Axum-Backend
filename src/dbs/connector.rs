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

#[cfg(test)]
mod tests {
    use super::DbConfig;
    use super::*;
    #[tokio::test]
    async fn test_connect_invalid_endpoint() {
        // Description: Validates that the connect function returns a `ConnectionError`
        // when the database endpoint is incorrect.
        // Reasoning: This is a common operational failure. The application must
        // handle it gracefully.
        let config = DbConfig {
            endpoint: "ws://localhost:9999".to_string(), // Invalid port
            namespace: "test".to_string(),
            database: "test".to_string(),
            username: "root".to_string(),
            password: "root".to_string(),
        };

        let result = connect(&config).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DatabaseError::ConnectionError(_)
        ));
    }
}
