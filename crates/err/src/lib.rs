mod app_error;
mod domain;

// Re-export main types
pub use app_error::AppError;
pub use domain::{DatabaseError, EnvironmentError, SshError};

pub type AppResult<T> = std::result::Result<T, AppError>;
