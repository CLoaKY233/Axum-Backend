use super::error::EnvironmentError;
use std::{env, str::FromStr};
use tracing::debug;

/// Retrieves a required environment variable as a String
/// # Errors
/// Returns `EnvironmentError::NotFoundError` if the variable is not set
pub fn get_required(key: &str) -> Result<String, EnvironmentError> {
    env::var(key).map_err(|_| EnvironmentError::NotFoundError(key.to_string()))
}

/// Retrieves an optional environment variable with a default value
#[must_use]
pub fn get_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        debug!(key = %key, default = %default, "Using default value for environment variable");
        default.to_string()
    })
}

/// Retrieves and parses an environment variable.
///
/// # Errors
///
/// - `EnvironmentError::NotFoundError` if the variable is not set.
/// - `EnvironmentError::ParseError` if the variable cannot be parsed.
pub fn get_parsed<T>(key: &str) -> Result<T, EnvironmentError>
where
    T: FromStr,
{
    let value = get_required(key)?;

    value
        .parse::<T>()
        .map_err(|_| EnvironmentError::ParseError {
            key: key.to_string(),
            value: value.clone(),
            type_name: std::any::type_name::<T>(),
        })
}

/// Retrieves and parses an environment variable with a default value
#[must_use]
pub fn get_parsed_or_default<T>(key: &str, default: T) -> T
where
    T: FromStr + std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or_else(|| {
            debug!(key = %key, default = ?default, "Using default parsed value for environment variable");
            default
        })
}

/// Retrieves a boolean environment variable
/// Accepts: true/false, 1/0, yes/no, on/off (case-insensitive)
#[must_use]
pub fn get_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .and_then(|v| match v.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_required_success() {
        unsafe { std::env::set_var("TEST_VAR", "test_value") };
        let result = get_required("TEST_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
        unsafe { std::env::remove_var("TEST_VAR") };
    }

    #[test]
    fn test_get_required_missing() {
        let result = get_required("NONEXISTENT_VAR");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvironmentError::NotFoundError(_)
        ));
    }

    #[test]
    fn test_get_or_default_existing() {
        unsafe { std::env::set_var("TEST_DEFAULT", "actual_value") };
        let result = get_or_default("TEST_DEFAULT", "default_value");
        assert_eq!(result, "actual_value");
        unsafe { std::env::remove_var("TEST_DEFAULT") };
    }

    #[test]
    fn test_get_or_default_missing() {
        let result = get_or_default("MISSING_VAR", "default_value");
        assert_eq!(result, "default_value");
    }

    #[test]
    fn test_get_parsed_success() {
        unsafe { std::env::set_var("TEST_PORT", "8080") };
        let result: Result<u16, EnvironmentError> = get_parsed("TEST_PORT");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8080);
        unsafe { std::env::remove_var("TEST_PORT") };
    }

    #[test]
    fn test_get_parsed_invalid_type() {
        unsafe { std::env::set_var("TEST_PORT", "invalid") };
        let result: Result<u16, EnvironmentError> = get_parsed("TEST_PORT");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvironmentError::ParseError { .. }
        ));
        unsafe { std::env::remove_var("TEST_PORT") };
    }

    #[test]
    fn test_get_parsed_or_default() {
        unsafe { std::env::set_var("TEST_NUM", "42") };
        let result: i32 = get_parsed_or_default("TEST_NUM", 100);
        assert_eq!(result, 42);
        unsafe { std::env::remove_var("TEST_NUM") };

        let result: i32 = get_parsed_or_default("MISSING_NUM", 100);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_get_bool_variations() {
        let test_cases = vec![
            ("true", true),
            ("TRUE", true),
            ("1", true),
            ("yes", true),
            ("on", true),
            ("false", false),
            ("FALSE", false),
            ("0", false),
            ("no", false),
            ("off", false),
        ];

        for (value, expected) in test_cases {
            unsafe { std::env::set_var("TEST_BOOL", value) };
            let result = get_bool("TEST_BOOL", false);
            assert_eq!(result, expected, "Failed for value: {value}");
            unsafe { std::env::remove_var("TEST_BOOL") };
        }

        // Test default
        let result = get_bool("MISSING_BOOL", true);
        assert!(result);
    }
}
