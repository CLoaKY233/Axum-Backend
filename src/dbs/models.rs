use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub type DbConnection = Arc<Surreal<Any>>;
pub struct Database {
    pub db: DbConnection,
}

#[derive(Clone)]
pub struct DbConfig {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}
