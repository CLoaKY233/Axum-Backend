use axum::{Router, routing::get};
use axum_backend::{
    AppError,
    dbs::models::DbConfig,
    sys::{
        config::state::AppState,
        health::{aggregator::aggregate_health, components::create_health_checkers},
        log,
    },
};
use std::sync::Arc;
use tokio::time::{Duration, timeout};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logging
    log::init();

    // Now we can use structured logging
    info!("🚀 Application starting");

    info!("Loading database configuration");
    let config = DbConfig::from_env()?;

    info!(
        endpoint = %config.endpoint,
        namespace = %config.namespace,
        database = %config.database,
        "Connecting to database"
    );

    let connection = timeout(Duration::from_secs(10), config.connect())
        .await
        .map_err(|_| {
            error!("Database connection timeout after 10s");
            AppError::ServerError("Database connection timeout".to_string())
        })??;

    info!("✅ Database connected successfully");

    let health_checkers = Arc::new(create_health_checkers(connection.clone()));
    let state = Arc::new(AppState {
        db_connection: connection,
        health_checkers,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(aggregate_health))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to bind to port 3000");
            e
        })?;

    info!("🚀 Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.map_err(|e| {
        error!(error = %e, "Server error");
        e
    })?;

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to Anubrahman"
}
