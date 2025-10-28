use thiserror::Error;

/// Database-specific errors.
#[derive(Debug, Error)]
pub enum DatabaseError {
    /// Failed to connect to database
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    /// Query execution failed
    #[error("Database query error: {0}")]
    QueryError(String),

    /// Authentication failed
    #[error("Database authentication error: {0}")]
    AuthenticationError(String),

    /// Resource not found in database
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Database configuration error
    #[error("Database configuration error: {0}")]
    ConfigError(String),
}

// Conversion from third-party errors
impl From<surrealdb::Error> for DatabaseError {
    fn from(err: surrealdb::Error) -> Self {
        // Map specific surrealdb errors to our error types
        Self::QueryError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppError;
    use axum::response::IntoResponse;

    #[test]
    fn test_database_error_display() {
        let error = DatabaseError::ConnectionError("Connection failed".to_string());
        assert_eq!(
            error.to_string(),
            "Database connection error: Connection failed"
        );
    }

    #[test]
    fn test_error_conversion_chain() {
        // Test: DatabaseError -> AppError -> Response
        let db_error = DatabaseError::NotFound("user".to_string());
        let app_error: AppError = db_error.into();
        let response = app_error.into_response();
        assert_eq!(response.status(), 404);
    }

    #[test]
    fn test_surrealdb_error_conversion() {
        // Test external error conversion
        let surreal_err = surrealdb::Error::Db(surrealdb::error::Db::QueryNotExecuted);
        let db_error: DatabaseError = surreal_err.into();
        assert!(matches!(db_error, DatabaseError::QueryError(_)));
    }
}
