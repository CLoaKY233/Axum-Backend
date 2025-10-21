
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum SshError {
    ConnectionFailed(String),
    AuthenticationFailed(String),
}

impl fmt::Display for SshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "SSH connection failed: {msg}"),
            Self::AuthenticationFailed(msg) => write!(f, "SSH authentication failed: {msg}"),
        }
    }
}

impl std::error::Error for SshError {}

impl IntoResponse for SshError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::AuthenticationFailed(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::ConnectionFailed(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
