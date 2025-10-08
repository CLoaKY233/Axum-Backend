use axum::routing::get;
use axum_backend::{
    AppError,
    sys::{health::aggregate_health, init::initialize},
};

use tracing::error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let (app, state, listener) = initialize().await?;

    // Add routes to the router
    let app = app
        .route("/", get(root))
        .route("/health", get(aggregate_health))
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
    "Welcome to Anubrahman"
}
