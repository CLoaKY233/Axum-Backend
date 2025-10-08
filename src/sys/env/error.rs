use std::fmt;

#[derive(Debug)]
pub enum EnvironmentError {
    NotFoundError(String),
    ParseError {
        key: String,
        value: String,
        type_name: &'static str,
    },
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundError(key) => {
                write!(f, "Environment variable '{key}' is not set")
            }
            Self::ParseError {
                key,
                value,
                type_name,
            } => {
                write!(f, "Failed to parse '{key}={value}' as {type_name}")
            }
        }
    }
}

impl std::error::Error for EnvironmentError {}
