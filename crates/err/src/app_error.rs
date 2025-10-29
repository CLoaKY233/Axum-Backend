//! Defines the primary `AppError` and its `IntoResponse` implementation.

use super::domain::{DatabaseError, EnvironmentError, SshError};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

/// The unified error type for the application.
#[derive(Debug, Error)]
pub enum AppError {
    /// Database-related errors.
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    /// SSH connection/operation errors.
    #[error("SSH error: {0}")]
    Ssh(#[from] SshError),

    /// Environment configuration errors.
    #[error("Environment error: {0}")]
    Environment(#[from] EnvironmentError),

    /// Generic server errors.
    #[error("Server error: {0}")]
    ServerError(String),

    /// Server binding errors.
    #[error("Bind error: {0}")]
    BindError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Extract status code and error details
        let (status, error_type, message) = self.get_response_parts();

        // Log the error with appropriate level
        match status {
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE => {
                error!(error = ?self, status = %status, "Critical error occurred");
            }
            _ => {
                tracing::warn!(error = ?self, status = %status, "Handled error occurred");
            }
        }

        // Create JSON response
        let body = Json(json!({
            "status": status.as_u16(),
            "error": error_type,
            "message": message,
        }));

        (status, body).into_response()
    }
}

impl AppError {
    /// Returns the appropriate HTTP status code and error details for the client.
    #[inline]
    fn get_response_parts(&self) -> (StatusCode, &'static str, &'static str) {
        match self {
            Self::Database(e) => match e {
                DatabaseError::ConnectionError(_) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "database_connection_error",
                    "Database service temporarily unavailable",
                ),
                DatabaseError::QueryError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_query_error",
                    "Database query failed",
                ),
                DatabaseError::AuthenticationError(_) => (
                    StatusCode::UNAUTHORIZED,
                    "database_auth_error",
                    "Database authentication failed",
                ),
                DatabaseError::NotFound(_) => (
                    StatusCode::NOT_FOUND,
                    "database_not_found",
                    "Resource not found",
                ),
                DatabaseError::ConfigError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_config_error",
                    "Database configuration error",
                ),
            },

            Self::Ssh(e) => match e {
                SshError::ConnectionFailed(_) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "ssh_connection_failed",
                    "SSH connection failed",
                ),
                SshError::AuthenticationFailed(_) => (
                    StatusCode::UNAUTHORIZED,
                    "ssh_auth_failed",
                    "SSH authentication failed",
                ),
                SshError::InternalTaskError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ssh_internal_error",
                    "SSH operation failed",
                ),
                SshError::TimeoutError(_) => (
                    StatusCode::REQUEST_TIMEOUT,
                    "ssh_connection_timeout",
                    "SSH connection timed out",
                ),
            },

            Self::Environment(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "configuration_error",
                "Application misconfiguration detected",
            ),

            Self::ServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Internal server error",
            ),

            Self::BindError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "bind_error",
                "Server startup failed",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_error_chain() {
        let db_error = DatabaseError::QueryError("Query failed".to_string());
        let app_error: AppError = db_error.into();

        // Test error source chain
        assert!(app_error.source().is_some());
        assert!(matches!(app_error, AppError::Database(_)));
    }

    #[test]
    fn test_error_display() {
        let error = AppError::ServerError("Internal error".to_string());
        assert_eq!(error.to_string(), "Server error: Internal error");
    }

    #[test]
    fn test_http_status_codes() {
        let tests = vec![
            (
                DatabaseError::NotFound("test".into()),
                StatusCode::NOT_FOUND,
            ),
            (
                DatabaseError::AuthenticationError("test".into()),
                StatusCode::UNAUTHORIZED,
            ),
            (
                DatabaseError::ConnectionError("test".into()),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
        ];

        for (db_error, expected_status) in tests {
            let app_error = AppError::Database(db_error);
            let (status, _, _) = app_error.get_response_parts();
            assert_eq!(status, expected_status);
        }
    }

    #[test]
    fn test_automatic_conversion() {
        // Test #[from] attribute works
        let _: AppError = DatabaseError::NotFound("test".into()).into();
        let _: AppError = SshError::TimeoutError("test".into()).into();
        let _: AppError = EnvironmentError::NotFoundError("test".into()).into();
    }
}
