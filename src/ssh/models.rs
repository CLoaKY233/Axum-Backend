use serde::{Deserialize, Serialize};
use std::fmt;

/// Credentials for an SSH connection.
#[derive(Deserialize, Clone)]
pub struct SshCredentials {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

/// The response indicating the connection status.
#[derive(Serialize)]
pub struct ConnectionStatus {
    pub status: String,
    pub message: String,
}

impl fmt::Debug for SshCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SshCredentials")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("user", &self.user)
            .field("password", &"***")
            .finish()
    }
}
