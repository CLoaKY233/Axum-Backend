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
