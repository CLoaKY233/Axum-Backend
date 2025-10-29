//! # Domain-Specific Errors
//!
//! This module aggregates and re-exports error types from different application
//! domains, such as database, environment, and SSH.

mod dbs;
mod env;
mod ssh;

pub use dbs::DatabaseError;
pub use env::EnvironmentError;
pub use ssh::SshError;
