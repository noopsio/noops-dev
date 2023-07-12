mod models;
mod schema;
pub mod wasmstore;

use std::path::Path;

use self::{
    models::*,
    schema::users::{self, dsl as users_dsl},
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

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

    pub fn get_user_by_id(&self, id: i32) -> anyhow::Result<Option<User>> {
        let mut connection = self.pool.get()?;
        let user = users_dsl::users
            .find(id)
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }

    pub fn get_user_by_gh_id(&self, id: i32) -> anyhow::Result<Option<User>> {
        let mut connection = self.pool.get()?;
        let user = users_dsl::users
            .filter(users_dsl::github_id.eq(id))
            .first::<User>(&mut connection)
            .optional()?;

        Ok(user)
    }

    pub fn create_user(
        &self,
        github_id: i32,
        email: &str,
        github_access_token: &str,
    ) -> anyhow::Result<User> {
        let new_user = NewUser {
            email,
            github_id,
            github_access_token,
        };

        let mut connection = self.pool.get()?;

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut connection)?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use tempfile::{tempdir, TempDir};

    const DATABASE_NAME: &str = "noops_test.sqlite";
    const EMAIL: &str = "test@example.com";
    const GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const GH_ID: i32 = 42;

    fn setup() -> anyhow::Result<(TempDir, Database)> {
        let temp_dir = tempdir()?;
        let database = Database::new(&temp_dir.path().join(DATABASE_NAME));
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        let mut connection = database.pool.get()?;
        connection.run_pending_migrations(migrations).unwrap();
        Ok((temp_dir, database))
    }

    #[test]
    fn create_user_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(GH_ID, EMAIL, GH_ACCESS_TOKEN)?;

        assert_eq!(EMAIL, user.email);
        assert_eq!(GH_ID, user.github_id);
        assert_eq!(GH_ACCESS_TOKEN, user.github_access_token);
        Ok(())
    }

    #[test]
    fn get_user_by_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(GH_ID, EMAIL, GH_ACCESS_TOKEN)?;
        let result = database.get_user_by_id(user.id)?;

        assert!(result.is_some());
        let result = result.unwrap();

        assert_eq!(user.id, result.id);
        assert_eq!(user.email, result.email);
        assert_eq!(user.github_id, result.github_id);
        assert_eq!(user.github_access_token, result.github_access_token);

        Ok(())
    }

    #[test]
    fn get_user_by_id_not_found() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let result = database.get_user_by_id(1)?;

        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn get_user_by_gh_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(GH_ID, EMAIL, GH_ACCESS_TOKEN)?;
        let result = database.get_user_by_gh_id(user.github_id)?;

        assert!(result.is_some());
        let result = result.unwrap();

        assert_eq!(user.id, result.id);
        assert_eq!(user.email, result.email);
        assert_eq!(user.github_id, result.github_id);
        assert_eq!(user.github_access_token, result.github_access_token);
        Ok(())
    }

    #[test]
    fn get_user_by_gh_id_not_found() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let result = database.get_user_by_gh_id(GH_ID)?;

        assert!(result.is_none());
        Ok(())
    }
}
