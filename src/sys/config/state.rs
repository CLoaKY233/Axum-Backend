use crate::{dbs::models::DbConnection, sys::health::models::HealthCheck};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: DbConnection,
    pub health_checkers: Arc<Vec<Box<dyn HealthCheck>>>,
}
