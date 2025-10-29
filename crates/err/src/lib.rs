//! # Application Error Handling
//!
//! This crate centralizes all application-specific errors. It provides a unified
//! `AppError` type and a consistent way to handle errors across the application.

mod app_error;
mod domain;

// Re-export main types
pub use app_error::AppError;
pub use domain::{DatabaseError, EnvironmentError, SshError};

/// A specialized `Result` for application-wide use.
pub type AppResult<T> = std::result::Result<T, AppError>;
