use crate::dbs::DatabaseError;
use crate::ssh::SshError;
use crate::sys::EnvironmentError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Database(DatabaseError),
    Ssh(SshError),
    ServerError(String),
    BindError(String),
    Environment(EnvironmentError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ssh(e) => write!(f, "SSH error: {e}"),
            Self::Database(e) => write!(f, "Database error: {e}"),
            Self::Environment(e) => write!(f, "Environment error: {e}"),
            Self::ServerError(msg) => write!(f, "Server error: {msg}"),
            Self::BindError(msg) => write!(f, "Bind error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Database(e) => Some(e),
            AppError::Ssh(e) => Some(e),
            AppError::Environment(e) => Some(e),
            _ => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            // Database errors
            Self::Database(e) => {
                error!(error = ?e, "Database error occurred");
                match e {
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
                }
            }

            // SSH errors
            Self::Ssh(e) => {
                error!(error = ?e, "SSH error occurred");
                match e {
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
                }
            }

            // Environment errors
            Self::Environment(e) => {
                error!(error = ?e, "Environment configuration error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "configuration_error",
                    "Application misconfiguration detected",
                )
            }

            // Server errors
            Self::ServerError(msg) => {
                error!(error = %msg, "Server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "server_error",
                    "Internal server error",
                )
            }

            // Bind errors
            Self::BindError(msg) => {
                error!(error = %msg, "Bind error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "bind_error",
                    "Server startup failed",
                )
            }
        };

        let body = Json(json!({
            "error": error_type,
            "message": message
        }));

        (status, body).into_response()
    }
}

// Keep all From implementations for automatic conversion
impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        Self::Database(err)
    }
}

impl From<EnvironmentError> for AppError {
    fn from(err: EnvironmentError) -> Self {
        Self::Environment(err)
    }
}

impl From<SshError> for AppError {
    fn from(err: SshError) -> Self {
        Self::Ssh(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::BindError(err.to_string())
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        Self::ServerError(format!("Environment variable error: {err}"))
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        Self::Database(DatabaseError::QueryError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_from_database_error() {
        let db_error = DatabaseError::QueryError("Query failed".to_string());
        let app_error: AppError = db_error.into();
        assert!(matches!(app_error, AppError::Database(_)));
    }

    #[test]
    fn test_app_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();
        assert!(matches!(app_error, AppError::BindError(_)));
    }

    #[test]
    fn test_app_error_display() {
        let error = AppError::ServerError("Server error".to_string());
        assert_eq!(error.to_string(), "Server error: Server error");
    }

    #[test]
    fn test_app_error_into_response() {
        let error = AppError::ServerError("Internal error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
