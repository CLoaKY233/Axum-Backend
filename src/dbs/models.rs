use std::fmt;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub type DbConnection = Arc<Surreal<Any>>;
pub struct Database {
    pub db: DbConnection,
}

#[derive(Clone)]
pub struct DbConfig {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl fmt::Debug for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database Configuration")
            .field("endpoint", &self.endpoint)
            .field("namespace", &self.namespace)
            .field("database", &self.database)
            .field("username", &"***")
            .field("password", &"***")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env as std_env;

    #[test]
    #[serial]
    fn test_dbconfig_from_env_success() {
        unsafe {
            std_env::set_var("DB_ENDPOINT", "ws://test.com");
            std_env::set_var("DB_NAMESPACE", "ns_test");
            std_env::set_var("DB_NAME", "db_test");
            std_env::set_var("DB_USERNAME", "user_test");
            std_env::set_var("DB_PASSWORD", "pass_test");
        }

        let config = DbConfig::from_env().expect("Should create config successfully");

        assert_eq!(config.endpoint, "ws://test.com");
        assert_eq!(config.namespace, "ns_test");
        assert_eq!(config.database, "db_test");
        assert_eq!(config.username, "user_test");
        assert_eq!(config.password, "pass_test");

        unsafe {
            std_env::remove_var("DB_ENDPOINT");
            std_env::remove_var("DB_NAMESPACE");
            std_env::remove_var("DB_NAME");
            std_env::remove_var("DB_USERNAME");
            std_env::remove_var("DB_PASSWORD");
        }
    }

    #[test]
    #[serial]
    fn test_dbconfig_from_env_missing_variable() {
        use err::EnvironmentError;

        unsafe {
            std_env::remove_var("DB_ENDPOINT");
            std_env::remove_var("DB_NAMESPACE");
            std_env::remove_var("DB_NAME");
            std_env::remove_var("DB_USERNAME");
            std_env::remove_var("DB_PASSWORD");
        }

        unsafe {
            std_env::set_var("DB_ENDPOINT", "ws://test.com");
            std_env::set_var("DB_NAMESPACE", "ns_test");
            // DB_NAME is missing
            std_env::set_var("DB_USERNAME", "user_test");
            std_env::set_var("DB_PASSWORD", "pass_test");
        }

        let result = DbConfig::from_env();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvironmentError::NotFoundError(_)
        ));

        unsafe {
            std_env::remove_var("DB_ENDPOINT");
            std_env::remove_var("DB_NAMESPACE");
            std_env::remove_var("DB_USERNAME");
            std_env::remove_var("DB_PASSWORD");
        }
    }

    #[test]
    fn test_dbconfig_debug_masks_credentials() {
        // Description: Ensures Debug output masks sensitive credentials
        // Reasoning: Prevents accidental credential leaks in logs
        let config = DbConfig {
            endpoint: "ws://test.com".to_string(),
            namespace: "ns_test".to_string(),
            database: "db_test".to_string(),
            username: "foo".to_string(),
            password: "bar".to_string(),
        };

        let debug_output = format!("{config:?}");

        assert!(debug_output.contains("ws://test.com"));
        assert!(debug_output.contains("ns_test"));
        assert!(debug_output.contains("db_test"));
        assert!(!debug_output.contains("foo"));
        assert!(!debug_output.contains("bar"));
        assert!(debug_output.contains("***"));
    }
}
