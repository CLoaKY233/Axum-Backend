use crate::sys::env;

use super::models::{LogConfig, LogFormat};

impl LogFormat {
    /// Creates a `LogFormat` from the `LOG_FORMAT` environment variable.
    pub fn from_env() -> Self {
        let format_str = env::get_or_default("LOG_FORMAT", "auto");

        match format_str.to_lowercase().as_str() {
            "json" => Self::Json,
            "compact" => Self::Compact,
            _ => {
                if cfg!(debug_assertions) {
                    Self::Compact
                } else {
                    Self::Json
                }
            }
        }
    }
}

impl LogConfig {
    /// Creates a `LogConfig` from environment variables.
    pub fn from_env() -> Self {
        let format = LogFormat::from_env();

        let filter = env::get_or_default(
            "RUST_LOG",
            if cfg!(debug_assertions) {
                "axum_backend=debug,tower_http=debug,info"
            } else {
                "axum_backend=info,tower_http=info,warn"
            },
        );

        Self { format, filter }
    }
}
