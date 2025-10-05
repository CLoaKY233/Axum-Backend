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
    let env_loaded = dotenvy::dotenv().is_ok();

    // Initialize logging
    log::init();
    if env_loaded {
        info!("Loaded .env file");
    } else {
        error!("No .env file found");
    }

    info!("Application is starting");

    info!("Loading database configuration from environment");
    let config = DbConfig::from_env()?;

    info!(
        endpoint = %config.endpoint,
        namespace = %config.namespace,
        database = %config.database,
        "Attempting to connect to the database"
    );

    let connection = timeout(Duration::from_secs(10), config.connect())
        .await
        .map_err(|_| {
            error!("Failed to connect to the database: connection timed out after 10 seconds");
            AppError::ServerError("Database connection timeout".to_string())
        })??;

    info!("Successfully connected to the database");

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
            error!(error = %e, "Failed to bind server to port 3000");
            e
        })?;

    info!("Server is listening for requests on http://0.0.0.0:3000");

    axum::serve(listener, app).await.map_err(|e| {
        error!(error = %e, "The server encountered an unrecoverable error");
        e
    })?;

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to Anubrahman"
}
