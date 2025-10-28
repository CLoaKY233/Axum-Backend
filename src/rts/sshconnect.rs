use crate::ssh::{ConnectionStatus, SshCredentials, ssh_connect};
use axum::Json;
use err::AppResult;

/// Handler for testing an SSH connection.
///
/// # Errors
/// Returns `AppError` if the SSH connection or authentication fails.
pub async fn ssh_handler(
    Json(credentials): Json<SshCredentials>,
) -> AppResult<Json<ConnectionStatus>> {
    let status = ssh_connect(&credentials).await?;
    Ok(Json(status))
}
