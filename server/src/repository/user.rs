use super::{
    create_id,
    schema::users::{self},
    Repository,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Identifiable, Insertable, Queryable, Selectable, Debug, Clone, PartialEq, Default)]
#[diesel(table_name = crate::repository::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub location: Option<String>,
    pub company: Option<String>,
    pub github_login: String,
    pub github_id: i32,
    pub github_access_token: String,
}

impl User {
    pub fn new(
        email: String,
        name: Option<String>,
        location: Option<String>,
        company: Option<String>,
        github_id: i32,
        github_login: String,
        github_access_token: String,
    ) -> Self {
        Self {
            id: create_id(),
            email,
            name,
            location,
            company,
            github_id,
            github_login,
            github_access_token,
        }
    }
}

#[cfg_attr(test, faux::create)]
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[cfg_attr(test, faux::methods)]
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

    fn delete(&self, _id: &str) -> anyhow::Result<()> {
        unimplemented!()
    }
}

#[cfg_attr(test, faux::methods)]
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
    const USER_NAME: &str = "user_name";
    const USER_LOCATION: &str = "Hamburg";
    const USER_COMPANY: &str = "Noops.io";
    const USER_GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const USER_GH_LOGIN: &str = "login_name";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref USER: User = User::new(
            USER_EMAIL.to_string(),
            Some(USER_NAME.to_string()),
            Some(USER_LOCATION.to_string()),
            Some(USER_COMPANY.to_string()),
            USER_GH_ID,
            USER_GH_LOGIN.to_string(),
            USER_GH_ACCESS_TOKEN.to_string()
        );
    }

    fn setup() -> anyhow::Result<(TempDir, UserRepository)> {
        let temp_dir = tempdir()?;
        let pool = create_pool(&temp_dir.path().join(DATABASE_NAME));
        let mut connection = pool.get()?;
        let users = UserRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
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
            Some(USER_NAME.to_string()),
            Some(USER_LOCATION.to_string()),
            Some(USER_COMPANY.to_string()),
            USER_GH_ID,
            USER_GH_LOGIN.to_string(),
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
