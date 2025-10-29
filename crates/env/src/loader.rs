use super::EnvResult;
use err::EnvironmentError;
use std::{env, str::FromStr};
use tracing::debug;

/// Retrieves a required environment variable as a String.
///
/// # Errors
///
/// Returns `EnvironmentError::NotFoundError` if the variable is not set.
pub fn get_required(key: &str) -> EnvResult<String> {
    env::var(key).map_err(|_| {
        debug!(key = %key, "Environment variable not found");
        EnvironmentError::NotFoundError(key.to_string())
    })
}

/// Retrieves an optional environment variable with a default value.
///
/// Logs when using default values for debugging purposes.
#[must_use]
pub fn get_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        debug!(
            key = %key,
            default = %default,
            "Using default value for environment variable"
        );
        default.to_string()
    })
}

/// Retrieves and parses an environment variable.
///
/// # Type Parameters
///
/// * `T` - Must implement `FromStr`
///
/// # Errors
///
/// - `EnvironmentError::NotFoundError` if the variable is not set
/// - `EnvironmentError::Parse` if the variable cannot be parsed
pub fn get_parsed<T>(key: &str) -> EnvResult<T>
where
    T: FromStr,
{
    let value = get_required(key)?;
    value.parse::<T>().map_err(|_| {
        debug!(
            key=%key,
            // value = %value,
            type_name=std::any::type_name::<T>(),
            "Failed to parse environment variable"
        );
        EnvironmentError::Parse {
            key: key.to_string(),
            value: value.clone(),
            type_name: std::any::type_name::<T>(),
        }
    })
}

/// Retrieves and parses an environment variable with a default value.
///
/// # Type Parameters
///
/// * `T` - Must implement `FromStr` and `Debug`
pub fn get_parsed_or_default<T>(key: &str, default: T) -> T
where
    T: FromStr + std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or_else(|| {
            debug!(key=%key,
                default = ?default,
                "Using default parsed value for environment variable");
            default
        })
}

/// Retrieves a boolean environment variable.
///
/// Accepts multiple formats (case-insensitive):
/// - `true`, `1`, `yes`, `on` → `true`
/// - `false`, `0`, `no`, `off` → `false`
#[must_use]
pub fn get_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .and_then(|v| match v.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => {
                debug!(
                    key = %key,
                    // value = %v,
                    "Invalid boolean value, using default"
                );
                None
            }
        })
        .unwrap_or_else(|| {
            debug!(
                key = %key,
                default = %default,
                "Boolean environment variable not found, using default"
            );
            default
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // A mutex to ensure that tests modifying the environment do not run concurrently.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    // Helper to set an environment variable for the duration of a test.
    // When the returned guard is dropped, the variable is unset.
    struct EnvVarGuard {
        key: String,
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            // The caller holds ENV_MUTEX while this guard exists, so cleanup is safe.
            unsafe {
                std::env::remove_var(&self.key);
            }
        }
    }

    fn set_test_var(key: &str, value: &str) -> EnvVarGuard {
        // The mutex must be locked before calling this function.
        unsafe {
            std::env::set_var(key, value);
        }
        EnvVarGuard {
            key: key.to_string(),
        }
    }

    #[test]
    fn test_get_required_success() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard = set_test_var("TEST_VAR", "test_value");

        let result = get_required("TEST_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
    }

    #[test]
    fn test_get_required_missing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Ensure the variable is not set
        unsafe {
            std::env::remove_var("NONEXISTENT_VAR");
        }

        let result = get_required("NONEXISTENT_VAR");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvironmentError::NotFoundError(_)
        ));
    }

    #[test]
    fn test_get_or_default_existing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard = set_test_var("TEST_DEFAULT", "actual_value");

        let result = get_or_default("TEST_DEFAULT", "default_value");
        assert_eq!(result, "actual_value");
    }

    #[test]
    fn test_get_or_default_missing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe {
            std::env::remove_var("MISSING_VAR");
        }

        let result = get_or_default("MISSING_VAR", "default_value");
        assert_eq!(result, "default_value");
    }

    #[test]
    fn test_get_parsed_success() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard = set_test_var("TEST_PORT", "8080");

        let result: Result<u16, EnvironmentError> = get_parsed("TEST_PORT");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8080);
    }

    #[test]
    fn test_get_parsed_invalid_type() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard = set_test_var("TEST_PORT", "invalid");

        let result: Result<u16, EnvironmentError> = get_parsed("TEST_PORT");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvironmentError::Parse { .. }
        ));
    }

    #[test]
    fn test_get_parsed_or_default() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard = set_test_var("TEST_NUM", "42");
        let result: i32 = get_parsed_or_default("TEST_NUM", 100);
        assert_eq!(result, 42);

        unsafe {
            std::env::remove_var("MISSING_NUM");
        }
        let result_missing: i32 = get_parsed_or_default("MISSING_NUM", 100);
        assert_eq!(result_missing, 100);

        let _guard2 = set_test_var("INVALID_NUM", "not-a-number");
        let result_invalid: i32 = get_parsed_or_default("INVALID_NUM", 100);
        assert_eq!(
            result_invalid, 100,
            "Should return default for unparsable value"
        );
    }

    #[test]
    fn test_get_bool_variations() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let test_cases = vec![
            ("true", true),
            ("TRUE", true),
            ("1", true),
            ("yes", true),
            ("YES", true),
            ("on", true),
            ("ON", true),
            ("false", false),
            ("FALSE", false),
            ("0", false),
            ("no", false),
            ("NO", false),
            ("off", false),
            ("OFF", false),
        ];

        for (value, expected) in test_cases {
            let _guard = set_test_var("TEST_BOOL", value);
            let result = get_bool("TEST_BOOL", false);
            assert_eq!(result, expected, "Failed for value: {value}");
        }

        // Test default
        unsafe {
            std::env::remove_var("MISSING_BOOL");
        }
        let result = get_bool("MISSING_BOOL", true);
        assert!(result);
    }

    #[test]
    fn test_get_bool_invalid_value() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard = set_test_var("TEST_BOOL", "maybe");
        let result = get_bool("TEST_BOOL", false);
        assert!(!result); // Should use default
    }

    #[test]
    fn test_multiple_types() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Test different numeric types
        let _g1 = set_test_var("TEST_U8", "255");
        let _g2 = set_test_var("TEST_I32", "-42");
        let _g3 = set_test_var("TEST_F64", "3.15");

        let u8_val: u8 = get_parsed("TEST_U8").unwrap();
        let i32_val: i32 = get_parsed("TEST_I32").unwrap();
        let f64_val: f64 = get_parsed("TEST_F64").unwrap();

        assert_eq!(u8_val, 255);
        assert_eq!(i32_val, -42);
        assert!((f64_val - 3.15).abs() < f64::EPSILON);
    }
}
