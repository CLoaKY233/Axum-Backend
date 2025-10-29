//! # Environment Variable Utilities
//!
//! This crate provides convenient functions for accessing and parsing
//! environment variables.

mod loader;
pub use err::EnvironmentError;
pub use loader::{get_bool, get_or_default, get_parsed, get_parsed_or_default, get_required};

/// A result type for environment variable operations.
pub type EnvResult<T> = std::result::Result<T, EnvironmentError>;
