use axum::routing::{get, post};
use axum_backend::{
    AppError,
    ssh::ssh_connection_handler,
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
        .route("/ssh/connect", post(ssh_connection_handler))
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
