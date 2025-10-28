use thiserror::Error;

/// Environment configuration errors.
#[derive(Debug, Error)]
pub enum EnvironmentError {
    /// Environment variable not found
    #[error("Environment variable '{0}' is not set")]
    NotFoundError(String),

    /// Failed to parse environment variable
    #[error("Failed to parse '{key}={value}' as {type_name}")]
    Parse {
        /// The environment variable key
        key: String,
        /// The value that failed to parse
        value: String,
        /// The expected type name
        type_name: &'static str,
    },
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
        assert_eq!(error.to_string(), "Failed to parse 'PORT=invalid' as u16");
    }
}
