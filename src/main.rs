use axum::routing::{get, post};
use axum_backend::{health_handler, initialize, root_handler, ssh_handler};
use err::{AppError, AppResult};

use tracing::error;

/// Initializes and runs the application.
///
/// # Errors
///
/// Returns `AppError` if initialization or server execution fails.
#[tokio::main]
async fn main() -> AppResult<()> {
    let (app, state, listener) = initialize().await?;

    // Add routes to the router
    let app = app
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/ssh/connect", post(ssh_handler))
        .with_state(state);

    // Start the server
    axum::serve(listener, app).await.map_err(|e| {
        error!(error = %e, "The server encountered an unrecoverable error");
        AppError::ServerError(e.to_string())
    })?;

    Ok(())
}
