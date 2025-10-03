use anubrahman_backend::{
    dbs::connector::{DbConfig, DbConnection},
    err::error::AppError,
};
use axum::{
    Router,
    extract::{Json, State},
    routing::get,
};
use dotenvy;
use serde::Serialize;
use std::sync::Arc;
use tokio::time::{Duration, timeout};

#[derive(Clone)]
struct AppState {
    db_connection: DbConnection,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    database: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Changed to AppError!
    dotenvy::dotenv().ok();

    println!("⏳ Loading configuration...");
    let config = DbConfig::from_env()?; // Auto-converts to AppError!

    println!("⏳ Connecting to database...");
    let connection = timeout(Duration::from_secs(10), config.connect())
        .await
        .map_err(|_| AppError::ServerError("Database connection timeout".to_string()))??;

    println!("✅ Database connected!");

    let state = Arc::new(AppState {
        db_connection: connection,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?; // Auto-converts io::Error to AppError!

    println!("🚀 Server running on http://localhost:3000");

    axum::serve(listener, app).await?; // Auto-converts io::Error to AppError!

    Ok(())
}

// ==================================

async fn root() -> &'static str {
    "Welcome to Anubrahman"
}

async fn health(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>, AppError> {
    timeout(
        Duration::from_secs(5),
        state.db_connection.query("return true;"),
    )
    .await
    .map_err(|_| AppError::ServerError("Health check timeout".to_string()))??;

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        database: "connected".to_string(),
    }))
}
