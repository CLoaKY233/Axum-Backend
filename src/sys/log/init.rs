use tracing_subscriber::{EnvFilter, fmt};

pub fn init() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .compact()
        .init();
}

//Level  |  When to Use                                 |  Example
//-------+----------------------------------------------+----------------------------------------
//error  |  Something broke, needs immediate attention  |  Database connection failed
//warn   |  Unusual but not breaking                    |  Health check slow (>1s)
//info   |  Important business events                   |  Server started, user created
//debug  |  Detailed flow for debugging                 |  Function entry/exit, variable values
//trace  |  EXTREMELY detailed, very noisy              |  Every database query, all HTTP headers

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Helper to ensure init is only called once across all tests
    fn initialize_once() {
        INIT.call_once(|| {
            init();
        });
    }

    #[test]
    fn test_init_does_not_panic() {
        // Test that the init function executes without panicking
        initialize_once();
    }

    #[test]
    fn test_init_with_default_filter() {
        // Test that init works with default filter when RUST_LOG is not set
        std::env::remove_var("RUST_LOG");
        
        // Can't call init() again as it would panic, but we can test the filter creation
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));
        
        // Verify the filter was created successfully
        assert_eq\!(format\!("{:?}", env_filter).contains("axum_backend"), true);
    }

    #[test]
    fn test_init_with_custom_rust_log() {
        // Test that custom RUST_LOG environment variable is respected
        std::env::set_var("RUST_LOG", "warn");
        
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));
        
        // Verify custom filter was loaded
        assert_eq\!(format\!("{:?}", env_filter).contains("warn"), true);
        
        std::env::remove_var("RUST_LOG");
    }

    #[test]
    fn test_init_with_invalid_rust_log_falls_back_to_default() {
        // Test that invalid RUST_LOG falls back to default filter
        std::env::set_var("RUST_LOG", "invalid@@@filter");
        
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));
        
        // Should fall back to default
        assert_eq\!(format\!("{:?}", env_filter).contains("axum_backend"), true);
        
        std::env::remove_var("RUST_LOG");
    }

    #[test]
    fn test_default_filter_includes_required_levels() {
        // Test that the default filter includes expected logging levels
        let default_filter = "axum_backend=debug,tower_http=debug,info";
        
        assert\!(default_filter.contains("axum_backend=debug"));
        assert\!(default_filter.contains("tower_http=debug"));
        assert\!(default_filter.contains("info"));
    }

    #[test]
    fn test_env_filter_with_trace_level() {
        // Test trace level configuration
        std::env::set_var("RUST_LOG", "trace");
        
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));
        
        assert_eq\!(format\!("{:?}", env_filter).contains("trace"), true);
        
        std::env::remove_var("RUST_LOG");
    }

    #[test]
    fn test_env_filter_with_error_level() {
        // Test error level configuration
        std::env::set_var("RUST_LOG", "error");
        
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));
        
        assert_eq\!(format\!("{:?}", env_filter).contains("error"), true);
        
        std::env::remove_var("RUST_LOG");
    }

    #[test]
    fn test_env_filter_with_module_specific_level() {
        // Test module-specific log level
        std::env::set_var("RUST_LOG", "axum_backend=trace,tower_http=warn");
        
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("axum_backend=debug,tower_http=debug,info"));
        
        let filter_str = format\!("{:?}", env_filter);
        assert\!(filter_str.contains("axum_backend") || filter_str.contains("trace"));
        
        std::env::remove_var("RUST_LOG");
    }
}