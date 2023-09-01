use super::{
    create_id,
    schema::projects::{self},
    user::User,
    Repository,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(
    Identifiable, Insertable, Queryable, Selectable, Associations, Debug, PartialEq, Clone,
)]
#[diesel(table_name = crate::repository::schema::projects)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: String,
    pub name: String,
    pub user_id: String,
}

impl Project {
    pub fn new(name: String, user_id: String) -> Self {
        Self {
            id: create_id(),
            name,
            user_id,
        }
    }
}

#[cfg_attr(test, faux::create)]
#[derive(Debug, Clone)]
pub struct ProjectRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[cfg_attr(test, faux::methods)]
impl Repository<Project> for ProjectRepository {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    fn read(&self, id: &str) -> anyhow::Result<Option<Project>> {
        let mut connection = self.pool.get()?;
        let project = projects::dsl::projects
            .find(id)
            .first::<Project>(&mut connection)
            .optional()?;

        Ok(project)
    }

    fn create(&self, project: &Project) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::insert_into(projects::table)
            .values(project)
            .execute(&mut connection)?;

        Ok(())
    }

    fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        diesel::delete(projects::table)
            .filter(projects::dsl::id.eq(id))
            .execute(&mut connection)?;

        Ok(())
    }
}

#[cfg_attr(test, faux::methods)]
impl ProjectRepository {
    pub fn belonging_to_by_name(
        &self,
        user: &User,
        project_name: &str,
    ) -> anyhow::Result<Option<Project>> {
        let mut connection = self.pool.get()?;
        let project = Project::belonging_to(user)
            .filter(projects::dsl::name.eq(project_name))
            .first::<Project>(&mut connection)
            .optional()?;

        Ok(project)
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
    const PROJECT_NAME: &str = "TEST_PROJECT";
    const USER_ID: &str = "oveethai3oophaV6Aiwei";

    const USER_EMAIL: &str = "test@example.com";
    const USER_NAME: &str = "user_name";
    const USER_LOCATION: &str = "Hamburg";
    const USER_COMPANY: &str = "Noops.io";
    const USER_GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const USER_GH_LOGIN: &str = "login_name";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref PROJECT: Project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
    }

    fn setup() -> anyhow::Result<(TempDir, ProjectRepository)> {
        let temp_dir = tempdir()?;
        let pool = create_pool(&temp_dir.path().join(DATABASE_NAME));
        let mut connection = pool.get()?;
        let projects = ProjectRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        connection.run_pending_migrations(migrations).unwrap();
        Ok((temp_dir, projects))
    }

    #[test]
    fn create_ok() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        projects.create(&PROJECT)?;

        Ok(())
    }

    #[test]
    fn create_conflict() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        projects.create(&PROJECT)?;
        let result = projects.create(&PROJECT);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn create_name_conflict() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        projects.create(&PROJECT)?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let result = projects.create(&project);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn read_by_id_ok() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        projects.create(&PROJECT)?;
        let result = projects.read(&PROJECT.id)?;

        assert!(result.is_some());
        let project = result.unwrap();
        assert_eq!(*PROJECT, project);

        Ok(())
    }

    #[test]
    fn delete_ok() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        projects.create(&PROJECT)?;
        projects.delete(&PROJECT.id)?;
        let result = projects.read(&PROJECT.id)?;

        assert!(result.is_none());
        Ok(())
    }

    #[test]
    #[ignore]
    fn delete_not_found() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        let result = projects.delete("UNKNOWN_PROJECT_ID");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn belonging_to_ok() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        let user = User::new(
            USER_EMAIL.to_string(),
            USER_NAME.to_string(),
            USER_LOCATION.to_string(),
            USER_COMPANY.to_string(),
            USER_GH_ID,
            USER_GH_LOGIN.to_string(),
            USER_GH_ACCESS_TOKEN.to_string(),
        );
        let mut project = PROJECT.clone();
        project.user_id = user.id.clone();
        projects.create(&project)?;

        let result = projects.belonging_to_by_name(&user, &project.name)?;
        assert!(result.is_some());
        let project_belonging_to = result.unwrap();
        assert_eq!(project, project_belonging_to);

        Ok(())
    }

    #[test]
    fn belonging_to_not_found() -> anyhow::Result<()> {
        let (_temp_dir, projects) = setup()?;
        let user = User::new(
            USER_EMAIL.to_string(),
            USER_NAME.to_string(),
            USER_LOCATION.to_string(),
            USER_COMPANY.to_string(),
            USER_GH_ID,
            USER_GH_LOGIN.to_string(),
            USER_GH_ACCESS_TOKEN.to_string(),
        );
        projects.create(&PROJECT)?;

        let result = projects.belonging_to_by_name(&user, &PROJECT.name)?;
        assert!(result.is_none());

        Ok(())
    }
}
