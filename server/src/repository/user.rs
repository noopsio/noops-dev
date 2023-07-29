use super::{
    create_id,
    schema::users::{self},
    Repository,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

<<<<<<< HEAD
#[derive(Identifiable, Insertable, Queryable, Selectable, Debug, Clone, PartialEq)]
=======
#[derive(Identifiable, Insertable, Queryable, Selectable, Debug, Clone, PartialEq, Default)]
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
#[diesel(table_name = crate::repository::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub email: String,
    pub github_id: i32,
    pub github_access_token: String,
}

impl User {
    pub fn new(email: String, github_id: i32, github_access_token: String) -> Self {
        Self {
            id: create_id(),
            email,
            github_id,
            github_access_token,
        }
    }
}

<<<<<<< HEAD
=======
#[cfg_attr(test, faux::create)]
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

<<<<<<< HEAD
=======
#[cfg_attr(test, faux::methods)]
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
impl Repository<User> for UserRepository {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    fn read(&self, id: &str) -> anyhow::Result<Option<User>> {
        let mut connection = self.pool.get()?;
        let user = users::dsl::users
            .find(id)
            .first::<User>(&mut connection)
            .optional()?;

        Ok(user)
    }

    fn create(&self, user: &User) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::insert_into(users::table)
            .values(user)
            .execute(&mut connection)?;
        Ok(())
    }

    fn delete(&self, _id: &str) -> anyhow::Result<User> {
        unimplemented!()
    }
}

<<<<<<< HEAD
=======
#[cfg_attr(test, faux::methods)]
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
impl UserRepository {
    pub fn read_by_gh_id(&self, github_id: i32) -> anyhow::Result<Option<User>> {
        let mut connection = self.pool.get()?;

        let user = users::dsl::users
            .filter(users::dsl::github_id.eq(github_id))
            .first::<User>(&mut connection)
            .optional()?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::create_pool;

    use super::*;
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use lazy_static::lazy_static;
    use tempfile::{tempdir, TempDir};

    const DATABASE_NAME: &str = "noops_test.sqlite";
    const USER_EMAIL: &str = "test@example.com";
    const USER_GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref USER: User = User::new(
            USER_EMAIL.to_string(),
            USER_GH_ID,
            USER_GH_ACCESS_TOKEN.to_string()
        );
    }

    fn setup() -> anyhow::Result<(TempDir, UserRepository)> {
        let temp_dir = tempdir()?;
        let pool = create_pool(&temp_dir.path().join(DATABASE_NAME));
<<<<<<< HEAD
        let users = UserRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        let mut connection = users.pool.get()?;
=======
        let mut connection = pool.get()?;
        let users = UserRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
        connection.run_pending_migrations(migrations).unwrap();
        Ok((temp_dir, users))
    }

    #[test]
    fn create_ok() -> anyhow::Result<()> {
        let (_temp_dir, users) = setup()?;
        let result = users.create(&USER);

        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn create_id_conflict() -> anyhow::Result<()> {
        let (_temp_dir, users) = setup()?;
        users.create(&USER)?;
        let result = users.create(&USER);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn create_gh_id_conflict() -> anyhow::Result<()> {
        let (_temp_dir, users) = setup()?;
        let user = User::new(
            USER_EMAIL.to_string(),
            USER_GH_ID,
            USER_GH_ACCESS_TOKEN.to_string(),
        );
        users.create(&user)?;
        let result = users.create(&USER);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn read_by_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, users) = setup()?;
        users.create(&USER)?;
        let result = users.read(&USER.id)?;

        assert!(result.is_some());
        let user = result.unwrap();
        assert_eq!(*USER, user);
        Ok(())
    }

    #[test]
    fn read_by_id_not_found() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let result = database.read(&USER.id)?;
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn read_by_gh_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, users) = setup()?;
        users.create(&USER)?;
        let result = users.read_by_gh_id(USER.github_id)?;

        assert!(result.is_some());
        let user = result.unwrap();
        assert_eq!(*USER, user);
        Ok(())
    }

    #[test]
    fn read_by_gh_id_not_found() -> anyhow::Result<()> {
        let (_temp_dir, users) = setup()?;
        let result = users.read_by_gh_id(USER_GH_ID)?;
        assert!(result.is_none());
        Ok(())
    }
}
