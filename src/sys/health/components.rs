use crate::{
    dbs::models::{Database, DbConnection},
    sys::health::models::HealthCheck,
};

/// Factory function to create all health check components
pub fn create_health_checkers(db_connection: DbConnection) -> Vec<Box<dyn HealthCheck>> {
    vec![Box::new(Database {
        db: db_connection.clone(),
    })]
}
