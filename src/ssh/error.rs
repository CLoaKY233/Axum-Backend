use std::fmt;

#[derive(Debug)]
pub enum SshError {
    ConnectionFailed(String),
    AuthenticationFailed(String),
    InternalTaskError(String),
}

impl fmt::Display for SshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "SSH connection failed: {msg}"),
            Self::AuthenticationFailed(msg) => write!(f, "SSH authentication failed: {msg}"),
            Self::InternalTaskError(msg) => write!(f, "Internal SSH task error: {msg}"),
        }
    }
}

impl std::error::Error for SshError {}
