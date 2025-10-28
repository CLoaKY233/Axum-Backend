# err

A comprehensive error handling crate for Axum-based applications, providing type-safe error types with automatic HTTP response conversion.

## Features

- **Domain-Specific Errors**: Organized error types for different application domains (Database, SSH, Environment)
- **Axum Integration**: Automatic conversion to HTTP responses with appropriate status codes
- **Type Safety**: Strong typing with `thiserror` for clear error hierarchies
- **Automatic Logging**: Built-in tracing integration for error logging
- **Result Type Alias**: Convenient `Result<T>` type for consistent error handling
- **Error Propagation**: Seamless error conversion using the `?` operator

## Usage

Add this crate to your workspace or local dependencies:

```toml
[dependencies]
err = { path = "../err" }
```

### Basic Example

```rust
use err::{AppError, DatabaseError, Result};

async fn get_user(id: u64) -> Result<User> {
    let user = database
        .query("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .await?; // DatabaseError automatically converts to AppError

    user.ok_or_else(|| DatabaseError::NotFound(format!("User {id}")).into())
}
```

### Axum Handler

```rust
use axum::{Json, extract::Path};
use err::Result;

async fn user_handler(
    Path(user_id): Path<u64>
) -> Result<Json<User>> {
    let user = get_user(user_id).await?;
    Ok(Json(user))
}
```

When an error occurs, it automatically converts to a JSON response:

```json
{
  "status": 404,
  "error": "database_not_found",
  "message": "Resource not found"
}
```

## Error Types

### AppError

The main error enum that wraps all domain-specific errors:

```rust
pub enum AppError {
    Database(DatabaseError),
    Ssh(SshError),
    Environment(EnvironmentError),
    ServerError(String),
    BindError(String),
}
```

### DatabaseError

Handles database-related errors:

- `ConnectionError` - Database connection failures (503 Service Unavailable)
- `QueryError` - Query execution failures (500 Internal Server Error)
- `AuthenticationError` - Database auth failures (401 Unauthorized)
- `NotFound` - Resource not found (404 Not Found)
- `ConfigError` - Database configuration issues (500 Internal Server Error)

**Example:**
```rust
use err::DatabaseError;

// Automatic conversion from SurrealDB errors
let result = db.query("SELECT * FROM users").await?;

// Manual error creation
return Err(DatabaseError::NotFound("User not found".into()).into());
```

### SshError

Handles SSH connection and operation errors:

- `ConnectionFailed` - SSH connection failures (503 Service Unavailable)
- `AuthenticationFailed` - SSH authentication failures (401 Unauthorized)
- `InternalTaskError` - SSH operation failures (500 Internal Server Error)
- `TimeoutError` - SSH operation timeouts (408 Request Timeout)

**Example:**
```rust
use err::SshError;

if !ssh_client.connect().await {
    return Err(SshError::ConnectionFailed("Host unreachable".into()).into());
}
```

### EnvironmentError

Handles environment configuration errors:

- `NotFoundError` - Missing environment variable (500 Internal Server Error)
- `Parse` - Environment variable parsing failures (500 Internal Server Error)

**Example:**
```rust
use err::EnvironmentError;

fn get_port() -> Result<u16> {
    let port_str = std::env::var("PORT")
        .map_err(|_| EnvironmentError::NotFoundError("PORT".into()))?;

    port_str.parse().map_err(|_| EnvironmentError::Parse {
        key: "PORT".into(),
        value: port_str,
        type_name: "u16",
    }.into())
}
```

## HTTP Status Code Mapping

| Error Type | Status Code | Error Type String |
|------------|-------------|-------------------|
| `DatabaseError::ConnectionError` | 503 | `database_connection_error` |
| `DatabaseError::QueryError` | 500 | `database_query_error` |
| `DatabaseError::AuthenticationError` | 401 | `database_auth_error` |
| `DatabaseError::NotFound` | 404 | `database_not_found` |
| `DatabaseError::ConfigError` | 500 | `database_config_error` |
| `SshError::ConnectionFailed` | 503 | `ssh_connection_failed` |
| `SshError::AuthenticationFailed` | 401 | `ssh_auth_failed` |
| `SshError::InternalTaskError` | 500 | `ssh_internal_error` |
| `SshError::TimeoutError` | 408 | `ssh_connection_timeout` |
| `EnvironmentError::*` | 500 | `configuration_error` |
| `ServerError` | 500 | `server_error` |
| `BindError` | 500 | `bind_error` |

## Logging

Errors are automatically logged with appropriate levels:

- **Critical errors** (500, 503): Logged at `ERROR` level
- **Client errors** (401, 404, 408): Logged at `WARN` level

```rust
// Automatic logging when error is converted to response
let response = app_error.into_response();
// Logs: error occurred with full debug information
```

## Result Type Alias

The crate provides a convenient `Result` type alias:

```rust
pub type Result<T> = std::result::Result<T, AppError>;
```

Use it throughout your application for consistency:

```rust
async fn process_data() -> Result<Data> {
    let db_data = fetch_from_db().await?;
    let processed = transform(db_data)?;
    Ok(processed)
}
```

## Testing

The crate includes comprehensive tests:

```bash
cargo test
```

Tests cover:
- Error conversion chains
- HTTP status code mapping
- Display implementations
- Automatic error conversions
- Integration with Axum responses

## Dependencies

- `axum` - Web framework integration
- `thiserror` - Error type derivation
- `serde` & `serde_json` - JSON response serialization
- `tracing` - Logging integration
- `surrealdb` - Database error conversion support

## License

See the root workspace for license information.
