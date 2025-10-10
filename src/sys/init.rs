use crate::{
    AppError,
    dbs::{
        connector::connect,
        models::{DbConfig, DbConnection},
    },
    init_tracing,
    sys::{
        config::{server::ServerConfig, state::AppState},
        env,
        health::components::create_health_checkers,
    },
};
use axum::Router;
use std::sync::Arc;
use tokio::time::{Duration, timeout};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

/// Loads and establishes a database connection.
///
/// # Errors
///
/// - `AppError::Database` if configuration is invalid or connection/authentication fails
/// - `AppError::ServerError` if the connection times out
pub async fn load_database() -> Result<DbConnection, AppError> {
    info!("Loading database configuration from environment");
    let config = DbConfig::from_env()?;

    info!(
        endpoint = %config.endpoint,
        namespace = %config.namespace,
        database = %config.database,
        "Attempting to connect to the database"
    );
    let timeout_secs = env::get_parsed_or_default("DB_CONNECTION_TIMEOUT", 10);
    let connection = timeout(Duration::from_secs(timeout_secs), connect(&config))
        .await
        .map_err(|_| {
            error!(
                "Failed to connect to the database: connection timed out after {} seconds",
                timeout_secs
            );
            AppError::ServerError(format!(
                "Database connection timeout after {timeout_secs} seconds"
            ))
        })??;

    info!("Successfully connected to the database");

    Ok(connection)
}

/// Loads environment variables from .env file
#[must_use]
pub fn load_env() -> bool {
    dotenvy::dotenv().is_ok()
}

/// Creates a TCP listener bound to the specified address.
///
/// # Errors
///
/// Returns `AppError::BindError` if the server fails to bind to the address,
/// for example if the port is already in use.
pub async fn load_listener(addr: &str) -> Result<tokio::net::TcpListener, AppError> {
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        error!(error = %e, "Failed to bind server to {}", addr);
        AppError::BindError(e.to_string())
    })?;

    info!("Server is listening for requests on http://{}", addr);

    Ok(listener)
}

pub fn load_router() -> Router<Arc<AppState>> {
    Router::new().layer(TraceLayer::new_for_http())
}

/// Initializes the application.
///
/// # Errors
///
/// - `AppError::Database` for database configuration or connection failures
/// - `AppError::ServerError` for connection timeouts
/// - `AppError::BindError` if the server fails to bind to its address
/// - `AppError::Environment` if required environment variables are missing
pub async fn initialize() -> Result<
    (
        Router<Arc<AppState>>,
        Arc<AppState>,
        tokio::net::TcpListener,
    ),
    AppError,
> {
    // Load environment variables
    let env_loaded = load_env();

    // Initialize tracing
    init_tracing();
    if env_loaded {
        info!("Loaded .env file");
    } else {
        error!("No .env file found");
    }

    info!(version = env!("CARGO_PKG_VERSION"), "Application");
    info!("Application is starting");

    // Load server configuration
    let server_config = ServerConfig::from_env();
    info!(
        host = %server_config.host,
        port = server_config.port,
        "Server configuration loaded"
    );

    // Load database connection
    let connection = load_database().await?;

    // Create health checkers
    let health_checkers = Arc::new(create_health_checkers(connection.clone()));

    // Create application state
    let state = Arc::new(AppState {
        db_connection: connection,
        health_checkers,
    });

    // Load router with state
    let router = load_router();

    // Load listener
    let listener = load_listener(&server_config.address()).await?;

    Ok((router, state, listener))
}
