use crate::dbs::connector::DbConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: DbConnection,
}
