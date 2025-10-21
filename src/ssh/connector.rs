use super::{
    error::SshError,
    models::{ConnectionStatus, SshCredentials},
};
use crate::AppError;
use std::net::TcpStream;
use tokio::task;
use tracing::{Level, debug, error, info, instrument, span};

/// Establishes a connection to the `SurrealDB` database.
///
/// # Errors
/// Returns `DatabaseError::ConnectionError` if the connection fails.
/// Returns `DatabaseError::AuthenticationError` if authentication fails.
#[instrument(
    name = "ssh_connect",
    skip(credentials),
    fields(
        ssh.user=%credentials.user,
        ssh.host=%credentials.host,
        ssh.port=%credentials.port,
    )
)]
pub async fn ssh_connect(credentials: &SshCredentials) -> Result<ConnectionStatus, AppError> {
    info!("Attempting to establish SSH connection.");

    // Clone credentials to move them into the blocking thread.
    let creds = credentials.clone();

    // Use spawn_blocking for the synchronous ssh2 operations.
    let result = task::spawn_blocking(move || {
        let span = span!(Level::INFO, "ssh_blocking_task");
        let _enter = span.enter();

        // 1. Establish a standard TCP connection.
        debug!("Establishing TCP connection...");
        let tcp = TcpStream::connect(format!("{}:{}", creds.host, creds.port))
            .map_err(|e| SshError::ConnectionFailed(format!("TCP connection failed: {e}")))?;
        debug!("TCP connection established.");

        // 2. Create a new SSH session.
        debug!("Creating SSH session...");
        let mut sess = ssh2::Session::new()
            .map_err(|e| SshError::ConnectionFailed(format!("Session creation failed: {e}")))?;
        sess.set_tcp_stream(tcp);
        debug!("SSH session created.");

        // 3. Perform the SSH handshake.
        debug!("Performing SSH handshake...");
        sess.handshake()
            .map_err(|e| SshError::ConnectionFailed(format!("SSH handshake failed: {e}")))?;
        debug!("SSH handshake successful.");

        // 4. Authenticate with username and password.
        debug!("Authenticating with user and password...");
        sess.userauth_password(&creds.user, &creds.password)
            .map_err(|e| SshError::AuthenticationFailed(format!("Authentication error: {e}")))?;
        debug!("Authentication successful.");

        // 5. Verify that authentication was successful.
        if !sess.authenticated() {
            return Err(SshError::AuthenticationFailed(
                "Invalid credentials or authentication method not supported.".to_string(),
            ));
        }

        info!("SSH connection and authentication successful.");
        Ok(ConnectionStatus {
            status: "success".to_string(),
            message: "Successfully connected and authenticated.".to_string(),
        })
    })
    .await;

    // Handle results from the blocking task.
    match result {
        Ok(Ok(status)) => Ok(status),
        Ok(Err(ssh_error)) => {
            error!(error.message = %ssh_error, "SSH connection logic failed.");
            Err(ssh_error.into())
        }
        Err(join_error) => {
            error!(error.message = %join_error, "Internal task error during SSH connection.");
            Err(SshError::InternalTaskError(join_error.to_string()).into())
        }
    }
}
