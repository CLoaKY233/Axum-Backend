use crate::dbs::error::DatabaseError;
use crate::sys::env::EnvironmentError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt::{self};

#[derive(Debug)]
pub enum AppError {
    // Database Errors
    Database(DatabaseError),

    // Server/IO Errors
    ServerError(String),
    BindError(String),

    // Environment Errors
    Environment(EnvironmentError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {e}"),
            Self::Environment(e) => write!(f, "Environment error: {e}"),
            Self::ServerError(msg) => write!(f, "Server error: {msg}"),
            Self::BindError(msg) => write!(f, "Bind error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // Delegate to DatabaseError's response
            Self::Database(db_err) => db_err.into_response(),

            // Environment errors at runtime (shouldn't normally happen)
            Self::Environment(env_err) => {
                let body = Json(json!({
                    "error": "configuration_error",
                    "message": env_err.to_string()
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
