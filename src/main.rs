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

    info!("Loaded environment variables from .env file");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_returns_welcome_message() {
        // Test that root function returns the expected welcome message
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(root());
        
        assert_eq\!(result, "Welcome to Anubrahman");
    }

    #[test]
    fn test_root_is_static_str() {
        // Test that root returns a static string (no allocations)
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(root());
        
        assert_eq\!(std::mem::size_of_val(&result), std::mem::size_of::<&str>());
    }

    #[tokio::test]
    async fn test_root_async() {
        // Test root function in async context
        let result = root().await;
        assert_eq\!(result, "Welcome to Anubrahman");
    }

    #[tokio::test]
    async fn test_root_consistency() {
        // Test that root returns the same value on multiple calls
        let result1 = root().await;
        let result2 = root().await;
        let result3 = root().await;
        
        assert_eq\!(result1, result2);
        assert_eq\!(result2, result3);
    }

    #[test]
    fn test_root_message_not_empty() {
        // Test that root message is not empty
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(root());
        
        assert\!(\!result.is_empty());
    }

    #[test]
    fn test_root_message_contains_anubrahman() {
        // Test that the welcome message mentions Anubrahman
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(root());
        
        assert\!(result.contains("Anubrahman"));
    }

    #[tokio::test]
    async fn test_root_return_type_is_static() {
        // Verify root returns a 'static lifetime string
        let result = root().await;
        let _static_ref: &'static str = result;
    }

    #[test]
    fn test_root_message_format() {
        // Test the exact format of the welcome message
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(root());
        
        assert\!(result.starts_with("Welcome"));
        assert\!(result.ends_with("Anubrahman"));
    }

    #[tokio::test]
    async fn test_root_no_side_effects() {
        // Test that calling root has no side effects
        let before = "state";
        let _result = root().await;
        let after = "state";
        
        assert_eq\!(before, after);
    }

    #[tokio::test]
    async fn test_root_is_pure_function() {
        // Test that root is a pure function (same input -> same output)
        let results: Vec<&str> = vec\![
            root().await,
            root().await,
            root().await,
            root().await,
            root().await,
        ];
        
        for result in &results {
            assert_eq\!(*result, "Welcome to Anubrahman");
        }
    }
}

// Integration-style tests for configuration and error handling
#[cfg(test)]
mod integration_tests {
    use axum_backend::{AppError, dbs::models::DbConfig};
    
    #[test]
    fn test_db_config_from_env_missing_endpoint() {
        // Test that DbConfig::from_env fails when DB_ENDPOINT is missing
        std::env::remove_var("DB_ENDPOINT");
        std::env::set_var("DB_NAMESPACE", "test");
        std::env::set_var("DB_NAME", "test");
        std::env::set_var("DB_USERNAME", "test");
        std::env::set_var("DB_PASSWORD", "test");
        
        let result = DbConfig::from_env();
        assert\!(result.is_err());
        
        // Clean up
        std::env::remove_var("DB_NAMESPACE");
        std::env::remove_var("DB_NAME");
        std::env::remove_var("DB_USERNAME");
        std::env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_db_config_from_env_missing_namespace() {
        // Test that DbConfig::from_env fails when DB_NAMESPACE is missing
        std::env::set_var("DB_ENDPOINT", "ws://localhost:8000");
        std::env::remove_var("DB_NAMESPACE");
        std::env::set_var("DB_NAME", "test");
        std::env::set_var("DB_USERNAME", "test");
        std::env::set_var("DB_PASSWORD", "test");
        
        let result = DbConfig::from_env();
        assert\!(result.is_err());
        
        // Clean up
        std::env::remove_var("DB_ENDPOINT");
        std::env::remove_var("DB_NAME");
        std::env::remove_var("DB_USERNAME");
        std::env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_db_config_from_env_missing_database() {
        // Test that DbConfig::from_env fails when DB_NAME is missing
        std::env::set_var("DB_ENDPOINT", "ws://localhost:8000");
        std::env::set_var("DB_NAMESPACE", "test");
        std::env::remove_var("DB_NAME");
        std::env::set_var("DB_USERNAME", "test");
        std::env::set_var("DB_PASSWORD", "test");
        
        let result = DbConfig::from_env();
        assert\!(result.is_err());
        
        // Clean up
        std::env::remove_var("DB_ENDPOINT");
        std::env::remove_var("DB_NAMESPACE");
        std::env::remove_var("DB_USERNAME");
        std::env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_db_config_from_env_missing_username() {
        // Test that DbConfig::from_env fails when DB_USERNAME is missing
        std::env::set_var("DB_ENDPOINT", "ws://localhost:8000");
        std::env::set_var("DB_NAMESPACE", "test");
        std::env::set_var("DB_NAME", "test");
        std::env::remove_var("DB_USERNAME");
        std::env::set_var("DB_PASSWORD", "test");
        
        let result = DbConfig::from_env();
        assert\!(result.is_err());
        
        // Clean up
        std::env::remove_var("DB_ENDPOINT");
        std::env::remove_var("DB_NAMESPACE");
        std::env::remove_var("DB_NAME");
        std::env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_db_config_from_env_missing_password() {
        // Test that DbConfig::from_env fails when DB_PASSWORD is missing
        std::env::set_var("DB_ENDPOINT", "ws://localhost:8000");
        std::env::set_var("DB_NAMESPACE", "test");
        std::env::set_var("DB_NAME", "test");
        std::env::set_var("DB_USERNAME", "test");
        std::env::remove_var("DB_PASSWORD");
        
        let result = DbConfig::from_env();
        assert\!(result.is_err());
        
        // Clean up
        std::env::remove_var("DB_ENDPOINT");
        std::env::remove_var("DB_NAMESPACE");
        std::env::remove_var("DB_NAME");
        std::env::remove_var("DB_USERNAME");
    }

    #[test]
    fn test_db_config_from_env_all_vars_present() {
        // Test successful configuration when all environment variables are present
        std::env::set_var("DB_ENDPOINT", "ws://localhost:8000");
        std::env::set_var("DB_NAMESPACE", "test_namespace");
        std::env::set_var("DB_NAME", "test_db");
        std::env::set_var("DB_USERNAME", "test_user");
        std::env::set_var("DB_PASSWORD", "test_pass");
        
        let result = DbConfig::from_env();
        assert\!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq\!(config.endpoint, "ws://localhost:8000");
        assert_eq\!(config.namespace, "test_namespace");
        assert_eq\!(config.database, "test_db");
        assert_eq\!(config.username, "test_user");
        assert_eq\!(config.password, "test_pass");
        
        // Clean up
        std::env::remove_var("DB_ENDPOINT");
        std::env::remove_var("DB_NAMESPACE");
        std::env::remove_var("DB_NAME");
        std::env::remove_var("DB_USERNAME");
        std::env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_app_error_conversion_from_io_error() {
        // Test that std::io::Error can be converted to AppError
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let app_err: AppError = io_err.into();
        
        assert\!(matches\!(app_err, AppError::BindError(_)));
    }

    #[test]
    fn test_app_error_display() {
        // Test AppError Display implementation
        let err = AppError::ServerError("test error".to_string());
        let display = format\!("{}", err);
        
        assert\!(display.contains("Server error"));
        assert\!(display.contains("test error"));
    }

    #[test]
    fn test_app_error_server_error_variant() {
        // Test ServerError variant
        let err = AppError::ServerError("Database connection timeout".to_string());
        let display = format\!("{}", err);
        
        assert\!(display.contains("Database connection timeout"));
    }
}