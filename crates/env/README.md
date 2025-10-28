# env

A Rust crate providing utilities for loading and parsing environment variables with comprehensive error handling and type safety.

## Features

- **Type-safe parsing**: Parse environment variables into any type implementing `FromStr`
- **Default value support**: Gracefully handle missing variables with sensible defaults
- **Boolean parsing**: Flexible boolean value parsing supporting multiple formats
- **Error handling**: Structured error types for missing or invalid variables
- **Debug logging**: Integrated tracing support for debugging configuration issues

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
env = { path = "path/to/env" }
```

## Usage

### Required Variables

```rust
use env::get_required;

fn main() -> env::Result<()> {
    let api_key = get_required("API_KEY")?;
    Ok(())
}
```

### Optional Variables with Defaults

```rust
use env::get_or_default;

let host = get_or_default("HOST", "localhost");
let database_url = get_or_default("DATABASE_URL", "postgres://localhost/db");
```

### Parsed Values

```rust
use env::{get_parsed, get_parsed_or_default};

// Parse required typed value
let port: u16 = get_parsed("PORT")?;

// Parse with default fallback
let workers: usize = get_parsed_or_default("WORKERS", 4);
```

### Boolean Values

```rust
use env::get_bool;

// Supports: true/false, 1/0, yes/no, on/off (case-insensitive)
let debug = get_bool("DEBUG", false);
let enable_cache = get_bool("ENABLE_CACHE", true);
```

## API Reference

### `get_required(key: &str) -> Result<String>`

Retrieves a required environment variable. Returns `EnvironmentError::NotFoundError` if not set.

### `get_or_default(key: &str, default: &str) -> String`

Retrieves an optional environment variable, returning the default value if not set.

### `get_parsed<T: FromStr>(key: &str) -> Result<T>`

Retrieves and parses an environment variable into the specified type. Returns `EnvironmentError::Parse` on parsing failure.

### `get_parsed_or_default<T: FromStr>(key: &str, default: T) -> T`

Retrieves and parses an environment variable with a fallback default value.

### `get_bool(key: &str, default: bool) -> bool`

Parses boolean environment variables with flexible format support.

## Error Handling

The crate provides structured error types through the `EnvironmentError` enum:

- `NotFoundError`: Environment variable not found
- `Parse`: Failed to parse variable into requested type

## Testing

Run the test suite:

```bash
cargo test
```

## License

See the workspace license file for details.
