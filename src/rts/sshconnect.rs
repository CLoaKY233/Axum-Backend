use crate::err::AppError;
use crate::ssh::{ConnectionStatus, SshCredentials, ssh_connect};
use axum::Json;

/// Handler for testing an SSH connection.
///
/// # Errors
/// Returns `AppError` if the SSH connection or authentication fails.
pub async fn ssh_handler(
    Json(credentials): Json<SshCredentials>,
) -> Result<Json<ConnectionStatus>, AppError> {
    let status = ssh_connect(&credentials).await?;
    Ok(Json(status))
}
