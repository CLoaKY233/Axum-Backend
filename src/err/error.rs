use crate::dbs::error::DatabaseError;
use crate::ssh::error::SshError;
use crate::sys::env::EnvironmentError;
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
        match self {
            // Delegate to DatabaseError's response
            Self::Database(db_err) => db_err.into_response(),
            Self::Ssh(ssh_err) => ssh_err.into_response(),
            // Environment errors at runtime (shouldn't normally happen)
            Self::Environment(env_err) => {
                error!(error = %env_err, "Environment configuration error");
                let body = Json(json!({
                    "error": "configuration_error",
                    "message": "Application misconfiguration detected. Check server logs."
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }

            // Handle Server errors
            Self::ServerError(msg) => {
                let body = Json(json!({
                    "error": "server_error",
                    "message": msg
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }

            Self::BindError(msg) => {
                let body = Json(json!({
                    "error": "bind_error",
                    "message": msg
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

// Automatically convert DatabaseError -> AppError
impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        Self::Database(err)
    }
}

// Automatically convert EnvironmentError -> AppError
impl From<EnvironmentError> for AppError {
    fn from(err: EnvironmentError) -> Self {
        Self::Environment(err)
    }
}

// Automatically convert ssh::SshError -> AppError
impl From<SshError> for AppError {
    fn from(err: SshError) -> Self {
        Self::Ssh(err)
    }
}

// Automatically convert io::Error -> AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::BindError(err.to_string())
    }
}

// Automatically convert env::VarError -> AppError
impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        Self::ServerError(format!("Environment variable error: {err}"))
    }
}

// To convert SurrealDB errors directly
impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        Self::Database(DatabaseError::QueryError(err.to_string()))
    }
}
