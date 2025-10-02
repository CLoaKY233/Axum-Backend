use super::error::DatabaseError;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Namespace;

pub type DbConnection = Arc<Surreal<Any>>;

pub async fn establish_connection(
    endpoint: &str,
    namespace: &str,
    database: &str,
    username: &str,
    password: &str,
) -> Result<DbConnection, DatabaseError> {
    let db = surrealdb::engine::any::connect(endpoint)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    db.use_ns(namespace)
        .use_db(database)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    db.signin(Namespace {
        namespace: namespace.into(),
        username: username.into(),
        password: password.into(),
    })
    .await
    .map_err(|e| DatabaseError::AuthenticationError(e.to_string()))?;

    Ok(Arc::new(db))
}

#[derive(Clone)]
pub struct DbConfig {
    pub endpoint: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, DatabaseError> {
        Ok(Self {
            endpoint: std::env::var("DB_ENDPOINT")
                .map_err(|_| DatabaseError::ConnectionError("DB_ENDPOINT not set".to_string()))?,

            namespace: std::env::var("DB_NAMESPACE")
                .map_err(|_| DatabaseError::ConnectionError("DB_NAMESPACE not set".to_string()))?,

            database: std::env::var("DB_NAME")
                .map_err(|_| DatabaseError::ConnectionError("DB_NAME not set".to_string()))?,

            username: std::env::var("DB_USERNAME")
                .map_err(|_| DatabaseError::ConnectionError("DB_USERNAME not set".to_string()))?,

            password: std::env::var("DB_PASSWORD")
                .map_err(|_| DatabaseError::ConnectionError("DB_PASSWORD not set".to_string()))?,
        })
    }

    pub async fn connect(&self) -> Result<DbConnection, DatabaseError> {
        establish_connection(
            &self.endpoint,
            &self.namespace,
            &self.database,
            &self.username,
            &self.password,
        )
        .await
    }
}
