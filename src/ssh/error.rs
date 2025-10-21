use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum SshError {
    ConnectionFailed(String),
    AuthenticationFailed(String),
    InternalTaskError(String),
}

impl fmt::Display for SshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "SSH connection failed: {msg}"),
            Self::AuthenticationFailed(msg) => write!(f, "SSH authentication failed: {msg}"),
            Self::InternalTaskError(msg) => write!(f, "Internal SSH task error: {msg}"),
        }
    }
}

impl std::error::Error for SshError {}

impl IntoResponse for SshError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            SshError::AuthenticationFailed(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            SshError::ConnectionFailed(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            SshError::InternalTaskError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
