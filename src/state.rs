use std::sync::{Arc, Mutex};
use sqlx::sqlite::SqlitePool;

///
/// The shared application state.
///
/// This struct is cloned for each request and can be accessed from handlers.
#[derive(Clone)]
pub struct AppState {
    pub view_count: Arc<Mutex<u64>>,
    pub db_pool: SqlitePool,
}

