mod function;
pub mod models;
mod project;
mod schema;
pub mod sqlite_uuid;
mod user;
pub mod wasmstore;

use std::path::Path;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn new(path: &Path) -> Self {
        let uri = path.to_string_lossy();
        let manager = ConnectionManager::<SqliteConnection>::new(uri);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .unwrap();
        Self { pool }
    }
}
