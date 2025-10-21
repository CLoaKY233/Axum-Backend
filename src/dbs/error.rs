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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_database_error_display() {
        let error = DatabaseError::ConnectionError("Connection failed".to_string());
        assert_eq!(error.to_string(), "Connection error: Connection failed");
        let error = DatabaseError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not found: User not found");
    }

    #[test]
    fn test_database_error_into_response() {
        let error = DatabaseError::AuthenticationError("Invalid credentials".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let error = DatabaseError::NotFound("Resource not found".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error = DatabaseError::ConnectionError("Connection failed".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
