use super::models::{LogConfig, LogFormat};
use std::env;
impl LogFormat {
    pub fn from_env() -> Self {
        env::var("LOG_FORMAT")
            .ok()
            .and_then(|s| match s.to_lowercase().as_str() {
                "json" => Some(Self::Json),
                "compact" => Some(Self::Compact),
                _ => None,
            })
            .unwrap_or_else(|| {
                // JSON for release builds, Compact for debug builds
                if cfg!(debug_assertions) {
                    Self::Compact
                } else {
                    Self::Json
                }
            })
    }
}

impl LogConfig {
    pub fn from_env() -> Self {
        let format = LogFormat::from_env();

        let filter = env::var("RUST_LOG").unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                "axum_backend=debug,tower_http=debug,info".to_string()
            } else {
                "axum_backend=info,tower_http=info,warn".to_string()
            }
        });

        Self { format, filter }
    }
}
