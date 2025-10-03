use anubrahman_backend::{
    AppError,
    dbs::connector::DbConfig,
    svr::{health::aggregate_health, state::AppState},
};

use axum::{Router, routing::get};
use dotenvy;
use std::sync::Arc;
use tokio::time::{Duration, timeout};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    println!("⏳ Loading configuration...");
    let config = DbConfig::from_env()?;

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
        .route("/health", get(aggregate_health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to Anubrahman"
}
