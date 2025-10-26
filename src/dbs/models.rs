use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub type DbConnection = Arc<Surreal<Any>>;
pub struct Database {
    pub db: DbConnection,
}

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbs::error::DatabaseError;
    use std::env as std_env;

    // Helper to set environment variables
    fn setup_db_env() {
        unsafe {
            std_env::set_var("DB_ENDPOINT", "ws://test.com");
            std_env::set_var("DB_NAMESPACE", "ns_test");
            std_env::set_var("DB_NAME", "db_test");
            std_env::set_var("DB_USERNAME", "user_test");
            std_env::set_var("DB_PASSWORD", "pass_test");
        }
    }

    // Helper to clear environment variables
    fn clear_db_env() {
        unsafe {
            std_env::remove_var("DB_ENDPOINT");
            std_env::remove_var("DB_NAMESPACE");
            std_env::remove_var("DB_NAME");
            std_env::remove_var("DB_USERNAME");
            std_env::remove_var("DB_PASSWORD");
        }
    }

    #[test]
    fn test_dbconfig_from_env_success() {
        // Description: Validates that `DbConfig` is correctly created when all
        // required environment variables are set.
        // Reasoning: This is the happy path for configuration loading.
        setup_db_env();

        let config = DbConfig::from_env().expect("Should create config successfully");

        assert_eq!(config.endpoint, "ws://test.com");
        assert_eq!(config.namespace, "ns_test");
        assert_eq!(config.database, "db_test");
        assert_eq!(config.username, "user_test");
        assert_eq!(config.password, "pass_test");

        clear_db_env();
    }

    #[test]
    fn test_dbconfig_from_env_missing_variable() {
        // Description: Validates that `DbConfig::from_env` returns a `ConfigError`
        // if a required environment variable is missing.
        // Reasoning: Ensures the application provides a clear error message on
        // misconfiguration instead of panicking.
        clear_db_env(); // Ensure all vars are unset

        unsafe {
            std_env::set_var("DB_ENDPOINT", "ws://test.com");
            std_env::set_var("DB_NAMESPACE", "ns_test");
            // DB_NAME is missing
            std_env::set_var("DB_USERNAME", "user_test");
            std_env::set_var("DB_PASSWORD", "pass_test");
        }

        let result = DbConfig::from_env();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DatabaseError::ConfigError(_)));

        clear_db_env();
    }
}
