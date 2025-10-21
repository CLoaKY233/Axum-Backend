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

// ✅ SINGLE, CONSISTENT IntoResponse implementation
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            // Database errors
            Self::Database(e) => match e {
                DatabaseError::ConnectionError(msg) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "database_connection_error",
                    msg,
                ),
                DatabaseError::QueryError(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_query_error",
                    msg,
                ),
                DatabaseError::AuthenticationError(msg) => {
                    (StatusCode::UNAUTHORIZED, "database_auth_error", msg)
                }
                DatabaseError::NotFound(msg) => (StatusCode::NOT_FOUND, "database_not_found", msg),
                DatabaseError::ConfigError(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_config_error",
                    msg,
                ),
            },

            // SSH errors
            Self::Ssh(e) => match e {
                SshError::ConnectionFailed(msg) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "ssh_connection_failed",
                    msg,
                ),
                SshError::AuthenticationFailed(msg) => {
                    (StatusCode::UNAUTHORIZED, "ssh_auth_failed", msg)
                }
                SshError::InternalTaskError(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "ssh_internal_error", msg)
                }
            },

            // Environment errors (internal, don't expose details)
            Self::Environment(e) => {
                error!(error = %e, "Environment configuration error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "configuration_error",
                    "Application misconfiguration detected. Check server logs.".to_string(),
                )
            }

            // Server errors
            Self::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "server_error", msg),

            // Bind errors
            Self::BindError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "bind_error", msg),
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
