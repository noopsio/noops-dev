use super::{models::*, schema::users, Database};
use crate::errors::Error::{self};
use diesel::prelude::*;

impl Database {
    pub fn read_user_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        let mut connection = self.pool.get().map_err(|err| anyhow::anyhow!(err))?;
        let user = users::dsl::users
            .find(id)
            .first::<User>(&mut connection)
            .optional()
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(user)
    }

    pub fn read_user_by_gh_id(&self, id: i32) -> anyhow::Result<Option<User>> {
        let mut connection = self.pool.get()?;
        let user = users::dsl::users
            .filter(users::dsl::github_id.eq(id))
            .first::<User>(&mut connection)
            .optional()
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(user)
    }

    pub fn create_user(
        &self,
        github_id: i32,
        email: String,
        github_access_token: String,
    ) -> anyhow::Result<User> {
        let user = User {
            id: Self::create_id(),
            email,
            github_id,
            github_access_token,
        };

        let mut connection = self.pool.get()?;

        let user = diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut connection)?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel_migrations::{FileBasedMigrations, MigrationHarness};
    use lazy_static::lazy_static;
    use tempfile::{tempdir, TempDir};

    const DATABASE_NAME: &str = "noops_test.sqlite";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref USER_EMAIL: String = "test@example.com".to_string();
        static ref USER_GH_ACCESS_TOKEN: String = "Yiu0Hae4ietheereij4OhneuNe6tae0e".to_string();
    }

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
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;

        assert_eq!(*USER_EMAIL, user.email);
        assert_eq!(USER_GH_ID, user.github_id);
        assert_eq!(*USER_GH_ACCESS_TOKEN, user.github_access_token);
        Ok(())
    }

    #[test]
    fn read_user_by_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;
        let result = database.read_user_by_id(&user.id)?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(user.id, result.id);
        assert_eq!(user.email, result.email);
        assert_eq!(user.github_id, result.github_id);
        assert_eq!(user.github_access_token, result.github_access_token);

        Ok(())
    }

    #[test]
    fn read_user_by_id_not_found() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let result = database.read_user_by_id("UNKNOWN_ID")?;
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn read_user_by_gh_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;
        let result = database.read_user_by_gh_id(user.github_id)?;

        assert!(result.is_some());
        let result = result.unwrap();

        assert_eq!(user.id, result.id);
        assert_eq!(user.email, result.email);
        assert_eq!(user.github_id, result.github_id);
        assert_eq!(user.github_access_token, result.github_access_token);
        Ok(())
    }

    #[test]
    fn read_user_by_gh_id_not_found() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let result = database.read_user_by_gh_id(USER_GH_ID)?;
        assert!(result.is_none());
        Ok(())
    }
}
