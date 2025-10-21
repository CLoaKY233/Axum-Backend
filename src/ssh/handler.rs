use super::connector::ssh_connect;
use super::models::{ConnectionStatus, SshCredentials};
use crate::AppError;
use axum::Json;

/// Handler for testing an SSH connection.
///
/// # Errors
/// Returns `AppError` if the SSH connection or authentication fails.
pub async fn ssh_connection_handler(
    Json(credentials): Json<SshCredentials>,
) -> Result<Json<ConnectionStatus>, AppError> {
    let status = ssh_connect(&credentials).await?;
    Ok(Json(status))
}
