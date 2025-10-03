use super::connector::DbConnection;
use super::error::DatabaseError;
use tokio::time::{Duration, timeout};

// Pure business logic - returns just a Result<String, DatabaseError>
pub async fn check_database_health(db: &DbConnection) -> Result<String, DatabaseError> {
    timeout(Duration::from_secs(5), db.query("return true;"))
        .await
        .map_err(|_| DatabaseError::ConnectionError("Health check timeout".to_string()))??;

    Ok("connected".to_string())
}
