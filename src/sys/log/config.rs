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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_format_from_env_json() {
        unsafe {
            std::env::set_var("LOG_FORMAT", "json");
        }
        let format = LogFormat::from_env();
        assert_eq!(format, LogFormat::Json);
        unsafe {
            std::env::remove_var("LOG_FORMAT");
        }
    }
    #[test]
    fn test_log_format_from_env_compact() {
        unsafe {
            std::env::set_var("LOG_FORMAT", "compact");
        }
        let format = LogFormat::from_env();
        assert_eq!(format, LogFormat::Compact);
        unsafe {
            std::env::remove_var("LOG_FORMAT");
        }
    }

    #[test]
    fn test_log_format_auto_debug_build() {
        unsafe {
            std::env::remove_var("LOG_FORMAT");
        }
        let format = LogFormat::from_env();
        if cfg!(debug_assertions) {
            assert_eq!(format, LogFormat::Compact);
        } else {
            assert_eq!(format, LogFormat::Json);
        }
    }

    #[test]
    fn test_log_config_from_env() {
        unsafe {
            std::env::set_var("LOG_FORMAT", "json");
            std::env::set_var("RUST_LOG", "info");
        }
        let config = LogConfig::from_env();
        assert_eq!(config.format, LogFormat::Json);
        assert_eq!(config.filter, "info");
        unsafe {
            std::env::remove_var("LOG_FORMAT");
            std::env::remove_var("RUST_LOG");
        }
    }
}
