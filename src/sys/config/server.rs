#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    /// Creates a `ServerConfig` from environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        let host = env::get_or_default("SERVER_HOST", "0.0.0.0");
        let port: u16 = env::get_parsed_or_default("SERVER_PORT", 3000);

        Self { host, port }
    }

    /// Returns the full address as a string (host:port).
    #[must_use]
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_from_env() {
        unsafe {
            std::env::set_var("SERVER_HOST", "127.0.0.1");
            std::env::set_var("SERVER_PORT", "8080");
        }
        let config = ServerConfig::from_env();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        unsafe {
            std::env::remove_var("SERVER_HOST");
            std::env::remove_var("SERVER_PORT");
        }
    }

    #[test]
    fn test_server_config_defaults() {
        unsafe {
            std::env::remove_var("SERVER_HOST");
            std::env::remove_var("SERVER_PORT");
        }
        let config = ServerConfig::from_env();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_server_config_address() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(config.address(), "127.0.0.1:8080");
    }
}
