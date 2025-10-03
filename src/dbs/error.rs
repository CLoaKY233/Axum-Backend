use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    AuthenticationError(String),
    NotFound(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            Self::QueryError(msg) => write!(f, "Query error: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::ConnectionError(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            Self::QueryError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };
        let body = Json(json!({"error":error_message}));
        (status, body).into_response()
    }
}

impl From<surrealdb::Error> for DatabaseError {
    fn from(err: surrealdb::Error) -> Self {
        Self::QueryError(err.to_string())
    }
}
