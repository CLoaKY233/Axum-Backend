

use serde::{Deserialize, Serialize};

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
