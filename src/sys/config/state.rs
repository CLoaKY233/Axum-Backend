use crate::dbs::DbConnection;
use hlt::HealthRegistry;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: DbConnection,
    pub health_registry: Arc<HealthRegistry>,
}
