use crate::{
    dbs::models::{Database, DbConnection},
    sys::health::models::HealthCheck,
};

/// Creates and returns a vector of all system health checkers.
///
/// This factory function initializes and collects all components that implement the
/// `HealthCheck` trait, such as database connections or external service clients.
/// Each checker is boxed and added to the vector, which can then be used by the
/// health aggregation service.
#[must_use = "health checkers should be registered or used"]
pub fn create_health_checkers(db_connection: DbConnection) -> Vec<Box<dyn HealthCheck>> {
    vec![Box::new(Database {
        db: db_connection,
        // No clone needed since db_connection is only used once.
        // When adding more components, clone all but the last:
        // Box::new(Database { db: db_connection.clone() }),
        // Box::new(Cache { db: db_connection }),
    })]
}
