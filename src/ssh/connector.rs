use super::models::{ConnectionStatus, SshCredentials};
use crate::sys::env;
use err::{AppResult, SshError};
use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};
use tokio::{task, time::timeout};
use tracing::{debug, error, info, instrument};

/// Establishes an SSH connection with layered timeouts.
///
/// This function orchestrates the entire connection process, wrapping the
/// synchronous, blocking `ssh2` logic within asynchronous boundaries and
/// applying multiple timeout layers for resilience.
///
/// # Timeouts
/// 1.  **Request Timeout (`SSH_REQUEST_TIMEOUT`):** An overall deadline for the
///     entire async function.
/// 2.  **Connection Timeout (`SSH_CONNECTION_TIMEOUT`):** For the initial TCP
///     socket connection.
/// 3.  **Operation Timeout (`SSH_OPERATION_TIMEOUT`):** For blocking I/O on
///     the socket and for SSH protocol-level operations (e.g., handshake).
///
/// # Errors
/// Returns an `AppError` wrapping a specific `SshError` variant if any part
/// of the process fails or times out.
#[instrument(
    name = "ssh_connect",
    skip(credentials),
    fields(
        ssh.user=%credentials.user,
        ssh.host=%credentials.host,
        ssh.port=%credentials.port,
    )
)]

pub async fn ssh_connect(credentials: &SshCredentials) -> AppResult<ConnectionStatus> {
    info!("Attempting to establish SSH connection.");

    // Clone credentials to move them into the blocking thread.
    let creds = credentials.clone();
    let request_timeout = env::get_parsed_or_default("SSH_REQUEST_TIMEOUT", 25);

    let task_result = timeout(
        Duration::from_secs(request_timeout),
        task::spawn_blocking(move || execute_blocking_ssh_connection(&creds)),
    )
    .await;

    match task_result {
        // The entire operation timed out.
        Err(_) => {
            let msg = format!("Request timed out after {request_timeout} seconds");
            error!(error.message = msg, "SSH connection failed.");
            Err(SshError::TimeoutError(msg).into())
        }
        // The blocking task completed (or panicked).
        Ok(join_result) => match join_result {
            Err(join_error) => {
                error!(error.message = %join_error, "Internal task error.");
                Err(SshError::InternalTaskError(join_error.to_string()).into())
            }
            Ok(ssh_result) => ssh_result.map_err(|e| {
                error!(error.message = %e, "SSH logic failed.");
                e.into()
            }),
        },
    }
}

/// Executes the synchronous (blocking) part of the SSH connection.
/// This function is intended to be run within `tokio::task::spawn_blocking`.
fn execute_blocking_ssh_connection(creds: &SshCredentials) -> Result<ConnectionStatus, SshError> {
    let conn_timeout = env::get_parsed_or_default("SSH_CONNECTION_TIMEOUT", 5);
    let op_timeout = env::get_parsed_or_default("SSH_OPERATION_TIMEOUT", 10);

    // 1. Resolve DNS. Can Block!
    let addr = (creds.host.as_str(), creds.port)
        .to_socket_addrs()
        .map_err(|e| SshError::ConnectionFailed(format!("DNS resolution failed: {e}")))?
        .next()
        .ok_or_else(|| SshError::ConnectionFailed("No socket address resolved".into()))?;

    // 2. Layer 2: TCP connection timeout.
    let tcp = TcpStream::connect_timeout(&addr, Duration::from_secs(conn_timeout))
        .map_err(|e| SshError::ConnectionFailed(format!("TCP connection failed: {e}")))?;

    // 3. Layer 2: Set I/O timeouts on the socket.
    tcp.set_read_timeout(Some(Duration::from_secs(op_timeout)))
        .map_err(|e| SshError::ConnectionFailed(format!("Failed to set read timeout: {e}")))?;
    tcp.set_write_timeout(Some(Duration::from_secs(op_timeout)))
        .map_err(|e| SshError::ConnectionFailed(format!("Failed to set write timeout: {e}")))?;

    debug!("TCP connection established.");

    let mut sess = ssh2::Session::new()
        .map_err(|e| SshError::ConnectionFailed(format!("Session creation failed: {e}")))?;
    sess.set_tcp_stream(tcp);

    // 4. Layer 1: Set protocol-level timeout.
    sess.set_timeout(u32::try_from(op_timeout.saturating_mul(1000)).unwrap_or(u32::MAX));

    debug!("Performing SSH handshake...");
    sess.handshake()
        .map_err(|e| SshError::ConnectionFailed(format!("Handshake failed: {e}")))?;
    debug!("SSH handshake successful.");

    debug!("Authenticating with user and password...");
    sess.userauth_password(&creds.user, &creds.password)
        .map_err(|e| SshError::AuthenticationFailed(format!("Authentication error: {e}")))?;

    if !sess.authenticated() {
        return Err(SshError::AuthenticationFailed(
            "Invalid credentials provided.".to_string(),
        ));
    }

    info!("SSH connection and authentication successful.");
    Ok(ConnectionStatus {
        status: "success".to_string(),
        message: "Successfully connected and authenticated.".to_string(),
    })
}
