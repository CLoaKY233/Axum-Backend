use crate::{
    dbs::models::{Database, DbConnection},
    sys::health::models::HealthCheck,
};

/// Factory function to create all health check components
pub fn create_health_checkers(db_connection: DbConnection) -> Vec<Box<dyn HealthCheck>> {
    vec![Box::new(Database {
        db: db_connection,
        // No clone needed since db_connection is only used once.
        // When adding more components, clone all but the last:
        // Box::new(Database { db: db_connection.clone() }),
        // Box::new(Cache { db: db_connection }),
    })]
}
