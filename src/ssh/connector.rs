use super::{
    error::SshError,
    models::{ConnectionStatus, SshCredentials},
};
use crate::AppError;
use std::net::TcpStream;
use tokio::task;

/// Establishes a connection to the `SurrealDB` database.
///
/// # Errors
/// Returns `DatabaseError::ConnectionError` if the connection fails.
/// Returns `DatabaseError::AuthenticationError` if authentication fails.
pub async fn connect(credentials: &SshCredentials) -> Result<ConnectionStatus, AppError> {
    // Clone credentials to move them into the separate blocking thread.
    let creds = credentials.clone();

    // Use spawn_blocking to run the synchronous ssh2 code without blocking the Axum server.
    let result = task::spawn_blocking(move || {
        // 1. Establish a standard TCP connection.
        let tcp = TcpStream::connect(format!("{}:{}", creds.host, creds.port))
            .map_err(|e| SshError::ConnectionFailed(format!("TCP connection failed: {e}")))?;

        // 2. Create a new SSH session from the TCP stream.
        let mut sess = ssh2::Session::new()
            .map_err(|e| SshError::ConnectionFailed(format!("Session creation failed: {e}")))?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| SshError::ConnectionFailed(format!("SSH handshake failed: {e}")))?;

        // 3. Authenticate with a username and password.
        sess.userauth_password(&creds.user, &creds.password)
            .map_err(|e| SshError::AuthenticationFailed(format!("Authentication error: {e}")))?;

        // 4. Verify that authentication was successful.
        if !sess.authenticated() {
            return Err(SshError::AuthenticationFailed(
                "Invalid credentials or authentication method not supported".to_string(),
            ));
        }

        // If all steps succeed, return the success status.
        Ok(ConnectionStatus {
            status: "success".to_string(),
            message: "Successfully connected and authenticated.".to_string(),
        })
    })
    .await;

    // Handle results from the blocking task.
    match result {
        Ok(Ok(status)) => Ok(status), // Task succeeded, and SSH connection was successful.
        Ok(Err(ssh_error)) => Err(ssh_error.into()), // Task succeeded, but SSH logic failed.
        Err(join_error) => {
            Err(SshError::ConnectionFailed(format!("Internal task error: {join_error}")).into())
        } // The task itself panicked or was cancelled.
    }
}
