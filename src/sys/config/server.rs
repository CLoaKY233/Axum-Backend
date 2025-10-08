use crate::sys::env;

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
