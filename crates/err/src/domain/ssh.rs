//! # SSH Errors
//!
//! Defines the `SshError` enum, which represents errors related to SSH
//! operations.

use thiserror::Error;

/// SSH-specific errors.
#[derive(Debug, Error)]
pub enum SshError {
    /// An SSH connection failed to establish.
    #[error("SSH connection failed: {0}")]
    ConnectionFailed(String),

    /// SSH authentication failed.
    #[error("SSH authentication failed: {0}")]
    AuthenticationFailed(String),

    /// An error occurred during an SSH task.
    #[error("Internal SSH task error: {0}")]
    InternalTaskError(String),

    /// An SSH operation timed out.
    #[error("SSH operation timed out: {0}")]
    TimeoutError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_error_display() {
        let error = SshError::ConnectionFailed("timeout".to_string());
        assert_eq!(error.to_string(), "SSH connection failed: timeout");
    }
}
