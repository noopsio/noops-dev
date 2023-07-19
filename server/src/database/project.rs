use super::{models::*, schema::projects, Database};
use crate::errors::Error::{self, ProjectAlreadyExists, ProjectNotFound};
use diesel::prelude::*;

impl Database {
    pub fn create_project(&self, user_id: String, project_name: &str) -> Result<Project, Error> {
        if self.read_project(&user_id, project_name)?.is_some() {
            return Err(ProjectAlreadyExists);
        }

        let project = Project {
            id: Self::create_id(),
            name: project_name.to_string(),
            user_id,
        };

        let mut connection = self.pool.get().map_err(|err| anyhow::anyhow!(err))?;

        let project = diesel::insert_into(projects::table)
            .values(&project)
            .returning(Project::as_returning())
            .get_result(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(project)
    }

    pub fn read_project(
        &self,
        user_id: &str,
        project_name: &str,
    ) -> anyhow::Result<Option<Project>> {
        let mut connection = self.pool.get()?;

        let project = projects::table
            .filter(projects::dsl::name.eq(project_name))
            .filter(projects::dsl::user_id.eq(user_id))
            .first::<Project>(&mut connection)
            .optional()?;

        Ok(project)
    }

    pub fn delete_project(&self, user_id: &str, project_name: &str) -> Result<Project, Error> {
        let mut connection = self.pool.get().map_err(|err| anyhow::anyhow!(err))?;

        self.read_project(user_id, project_name)?
            .ok_or(ProjectNotFound)?;

        let query = projects::table
            .filter(projects::user_id.eq(user_id))
            .filter(projects::dsl::name.eq(project_name));

        let project = diesel::delete(query)
            .get_result::<Project>(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;
        Ok(project)
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

    const PROJECT_NAME: &str = "TEST_PROJECT";

    fn setup() -> anyhow::Result<(TempDir, Database)> {
        let temp_dir = tempdir()?;
        let database = Database::new(&temp_dir.path().join(DATABASE_NAME));
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        let mut connection = database.pool.get()?;
        connection.run_pending_migrations(migrations).unwrap();
        Ok((temp_dir, database))
    }

    #[test]
    fn create_project_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;

        let project = database.create_project(user.id.clone(), PROJECT_NAME)?;

        assert_eq!(PROJECT_NAME, project.name);
        assert_eq!(user.id, project.user_id);

        Ok(())
    }

    #[test]
    fn create_project_conflict() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;

        database.create_project(user.id.clone(), PROJECT_NAME)?;
        let result = database.create_project(user.id, PROJECT_NAME);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn read_project_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;
        let project = database.create_project(user.id.clone(), PROJECT_NAME)?;

        let result = database.read_project(&user.id, PROJECT_NAME)?;

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(project.id, result.id);
        assert_eq!(project.name, result.name);
        assert_eq!(user.id, result.user_id);

        Ok(())
    }

    #[test]
    fn delete_project_ok() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;
        let _ = database.create_project(user.id.clone(), PROJECT_NAME)?;
        database.delete_project(&user.id, PROJECT_NAME)?;
        Ok(())
    }

    #[test]
    fn delete_project_not_found() -> anyhow::Result<()> {
        let (_temp_dir, database) = setup()?;
        let user = database.create_user(
            USER_GH_ID,
            (*USER_EMAIL).clone(),
            (*USER_GH_ACCESS_TOKEN).clone(),
        )?;
        let _ = database.create_project(user.id.clone(), PROJECT_NAME)?;
        database.delete_project(&user.id, PROJECT_NAME)?;
        let result = database.delete_project(&user.id, PROJECT_NAME);
        // TODO Check which error
        assert!(result.is_err());
        Ok(())
    }
}
