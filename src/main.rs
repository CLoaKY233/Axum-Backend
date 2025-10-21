use axum::{
    Json,
    routing::{get, post},
};
use axum_backend::{
    AppError,
    ssh::{
        connector::connect as ssh_connect,
        models::{ConnectionStatus, SshCredentials},
    },
    sys::{health::aggregate_health, init::initialize},
};

use tracing::error;

/// Initializes and runs the application.
///
/// # Errors
///
/// Returns `AppError` if initialization or server execution fails.
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let (app, state, listener) = initialize().await?;

    // Add routes to the router
    let app = app
        .route("/", get(root))
        .route("/health", get(aggregate_health))
        .route("/ssh/connect", post(test_ssh_connection_handler))
        .with_state(state);

    // Start the server
    axum::serve(listener, app).await.map_err(|e| {
        error!(error = %e, "The server encountered an unrecoverable error");
        AppError::ServerError(e.to_string())
    })?;

    Ok(())
}

/// The root endpoint of the application.
async fn root() -> &'static str {
    "Welcome to the system"
}

/// Handler for testing an SSH connection.
async fn test_ssh_connection_handler(
    Json(credentials): Json<SshCredentials>,
) -> Result<Json<ConnectionStatus>, AppError> {
    let status = ssh_connect(&credentials).await?;
    Ok(Json(status))
}
