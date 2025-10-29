//! # Environment Errors
//!
//! Defines the `EnvironmentError` enum, which represents errors related to
//! environment variable loading and parsing.

use std::fmt;
use thiserror::Error;

/// Environment configuration errors.
#[derive(Error)]
pub enum EnvironmentError {
    /// An environment variable was not found.
    #[error("Environment variable '{0}' is not set")]
    NotFoundError(String),

    /// An environment variable could not be parsed.
    #[error("Failed to parse '{key}' as {type_name}")]
    Parse {
        /// The environment variable key.
        key: String,
        /// The value that failed to parse.
        value: String,
        /// The expected type name.
        type_name: &'static str,
    },
}

impl fmt::Debug for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundError(key) => f.debug_tuple("NotFoundError").field(key).finish(),
            Self::Parse {
                key,
                value: _,
                type_name,
            } => f
                .debug_struct("Parse")
                .field("key", key)
                .field("value", &"[REDACTED]")
                .field("type_name", type_name)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_error_display() {
        let error = EnvironmentError::NotFoundError("DATABASE_URL".to_string());
        assert_eq!(
            error.to_string(),
            "Environment variable 'DATABASE_URL' is not set"
        );
    }

    #[test]
    fn test_parse_error_display() {
        let error = EnvironmentError::Parse {
            key: "PORT".to_string(),
            value: "invalid".to_string(),
            type_name: "u16",
        };
        assert_eq!(error.to_string(), "Failed to parse 'PORT' as u16");
    }
}
