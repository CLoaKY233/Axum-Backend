use env::{get_bool, get_or_default, get_parsed, get_parsed_or_default, get_required};
use std::sync::Mutex;

// A mutex to ensure that tests modifying the environment do not run concurrently.
static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_full_workflow() {
    let _lock = ENV_MUTEX.lock().unwrap();

    unsafe {
        std::env::set_var("APP_NAME", "test_app");
        std::env::set_var("PORT", "3000");
        std::env::set_var("DEBUG", "true");
    }

    // Test required
    let app_name = get_required("APP_NAME").unwrap();
    assert_eq!(app_name, "test_app");

    // Test parsed
    let port: u16 = get_parsed("PORT").unwrap();
    assert_eq!(port, 3000);

    // Test boolean
    let debug = get_bool("DEBUG", false);
    assert!(debug);

    // Test default
    let host = get_or_default("HOST", "localhost");
    assert_eq!(host, "localhost");

    // Test parsed with default
    let workers: usize = get_parsed_or_default("WORKERS", 4);
    assert_eq!(workers, 4);

    unsafe {
        std::env::remove_var("APP_NAME");
        std::env::remove_var("PORT");
        std::env::remove_var("DEBUG");
    }
}
