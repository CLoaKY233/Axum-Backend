use crate::dbs::models::DbConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: DbConnection,
}
