#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogFormat {
    Json,
    Compact,
}
pub struct LogConfig {
    pub format: LogFormat,
    pub filter: String,
}
//Level  |  When to Use                                 |  Example
//-------+----------------------------------------------+----------------------------------------
//error  |  Something broke, needs immediate attention  |  Database connection failed
//warn   |  Unusual but not breaking                    |  Health check slow (>1s)
//info   |  Important business events                   |  Server started, user created
//debug  |  Detailed flow for debugging                 |  Function entry/exit, va riable values
//trace  |  EXTREMELY detailed, very noisy              |  Every database query, all HTTP headers
