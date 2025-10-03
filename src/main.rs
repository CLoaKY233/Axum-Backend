use anubrahman_backend::dbs::{
    connector::{DbConfig, DbConnection},
    error::DatabaseError,
};
use axum::{
    Router,
    extract::{Json, State},
    routing::get,
};
use dotenvy;
use std::sync::Arc;
#[derive(Clone)]
struct AppState {
    db_connection: DbConnection,
}
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    dotenvy::dotenv().expect("Failed to read .env file");
    let config = DbConfig::from_env()?;
    let connection = config.connect().await?;
    let state = Arc::new(AppState {
        db_connection: connection,
    });

    let app = Router::new()
        .route("/", get(|| async { "Welcome to Anubrahman" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port to 3000");
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
    Ok(())
}
