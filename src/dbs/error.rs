use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    AuthenticationError(String),
    NotFound(String),
    ConfigError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Connection error: {msg}"),
            Self::QueryError(msg) => write!(f, "Query error: {msg}"),
            Self::AuthenticationError(msg) => write!(f, "Authentication error: {msg}"),

            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<surrealdb::Error> for DatabaseError {
    fn from(err: surrealdb::Error) -> Self {
        Self::QueryError(err.to_string())
    }
}
