pub mod function;
pub mod project;
pub mod schema;
pub mod user;

use self::{function::FunctionRepository, project::ProjectRepository, user::UserRepository};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::path::Path;

pub trait Repository<T> {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self;
    fn read(&self, id: &str) -> anyhow::Result<Option<T>>;
    fn create(&self, element: &T) -> anyhow::Result<()>;
    fn delete(&self, id: &str) -> anyhow::Result<()>;
}

pub fn create_pool(path: &Path) -> Pool<ConnectionManager<SqliteConnection>> {
    let uri = path.to_string_lossy();
    let manager = ConnectionManager::<SqliteConnection>::new(uri);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .unwrap()
}

pub fn new(path: &Path) -> (UserRepository, ProjectRepository, FunctionRepository) {
    let pool = create_pool(path);

    (
        UserRepository::new(pool.clone()),
        ProjectRepository::new(pool.clone()),
        FunctionRepository::new(pool),
    )
}

pub fn create_id() -> String {
    nanoid::nanoid!()
}
