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

/// Retrieves and parses an environment variable to any type implementing `FromStr`
/// # Errors
/// Returns `EnvironmentError` if the variable is not set or cannot be parsed
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
