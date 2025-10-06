use super::models::{LogConfig, LogFormat};
use tracing_subscriber::{EnvFilter, fmt};

/// Initializes the tracing subscriber for logging.
pub fn init_tracing() {
    let config = LogConfig::from_env();
    let env_filter = EnvFilter::try_new(&config.filter).unwrap_or_else(|_| EnvFilter::new("info"));

    match config.format {
        LogFormat::Json => {
            fmt()
                .json()
                .with_env_filter(env_filter)
                .with_target(true)
                .with_line_number(false)
                .with_file(false)
                .with_thread_ids(false)
                .with_level(true)
                .with_current_span(true)
                .init();
        }
        LogFormat::Compact => {
            fmt()
                .compact()
                .with_env_filter(env_filter)
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_thread_ids(false)
                .with_level(true)
                .init();
        }
    }

    tracing::info!(
        format = ?config.format,
        filter = %config.filter,
        "Tracing initialized"
    );
}
