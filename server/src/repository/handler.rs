use super::{
    create_id,
    project::Project,
    schema::handlers::{self, dsl},
    Repository,
};
use anyhow;
use common::dtos::Language;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(
    Identifiable,
    Insertable,
    Queryable,
    Selectable,
    Associations,
    Debug,
    Clone,
    PartialEq,
    AsChangeset,
)]
#[diesel(table_name = crate::repository::schema::handlers)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Handler {
    pub id: String,
    pub name: String,
    pub language: Language,
    pub hash: String,
    pub project_id: String,
}

impl Handler {
    pub fn new(name: String, language: Language, hash: String, project_id: String) -> Self {
        Self {
            id: create_id(),
            name,
            language,
            hash,
            project_id,
        }
    }
}

#[cfg_attr(test, faux::create)]
#[derive(Debug, Clone)]
pub struct HandlerRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[cfg_attr(test, faux::methods)]
impl Repository<Handler> for HandlerRepository {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    fn create(&self, handler: &Handler) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::insert_into(handlers::table)
            .values(handler)
            .on_conflict((dsl::name, dsl::project_id))
            .do_update()
            .set(handler)
            .execute(&mut connection)?;

        Ok(())
    }

    fn read(&self, id: &str) -> anyhow::Result<Option<Handler>> {
        let mut connection = self.pool.get()?;

        let handler = handlers::table
            .find(id)
            .first::<Handler>(&mut connection)
            .optional()?;

        Ok(handler)
    }

    fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::delete(handlers::table.find(id))
            .execute(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(())
    }
}

#[cfg_attr(test, faux::methods)]
impl HandlerRepository {
    pub fn belonging_to(&self, project: &Project) -> anyhow::Result<Vec<Handler>> {
        let mut connection = self.pool.get()?;
        let handlers = Handler::belonging_to(project).load::<Handler>(&mut connection)?;
        Ok(handlers)
    }

    pub fn belonging_to_by_name(
        &self,
        project: &Project,
        handler_name: &str,
    ) -> anyhow::Result<Option<Handler>> {
        let mut connection = self.pool.get()?;

        let handler = Handler::belonging_to(project)
            .filter(dsl::name.eq(handler_name))
            .first::<Handler>(&mut connection)
            .optional()?;

        Ok(handler)
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
    const HANDLER_NAME: &str = "FUNCTION_NAME";
    const HANDLER_HASH: &str = "aeHae9shaiquu4Iey8phishohqu1ON9y";
    const PROJECT_ID: &str = "xiekaiphoe7Luk3zeuNie";
    const PROJECT_NAME: &str = "PROJECT_NAME";
    const USER_ID: &str = "puphoonoh1bae6Binaixu";
    const HANDLER_LANGUAGE: Language = Language::Rust;

    lazy_static! {
        static ref HANDLER: Handler = Handler::new(
            HANDLER_NAME.to_string(),
            HANDLER_LANGUAGE,
            HANDLER_HASH.to_string(),
            PROJECT_ID.to_string()
        );
    }

    fn setup() -> anyhow::Result<(TempDir, HandlerRepository)> {
        let temp_dir = tempdir()?;
        let pool = create_pool(&temp_dir.path().join(DATABASE_NAME));
        let mut connection = pool.get()?;
        let projects = HandlerRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        connection.run_pending_migrations(migrations).unwrap();
        Ok((temp_dir, projects))
    }

    #[test]
    fn create_ok() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let result = handlers.create(&HANDLER);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn create_conflict() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        handlers.create(&HANDLER)?;
        let result = handlers.create(&HANDLER);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn create_name_conflict() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        handlers.create(&HANDLER)?;
        let handler = Handler::new(
            HANDLER_NAME.to_string(),
            HANDLER_LANGUAGE,
            HANDLER_HASH.to_string(),
            PROJECT_ID.to_string(),
        );
        let result = handlers.create(&handler);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn read_ok() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        handlers.create(&HANDLER)?;
        let handler = handlers.read(&HANDLER.id)?;

        assert!(handler.is_some());
        let handler = handler.unwrap();
        assert_eq!(*HANDLER, handler);
        Ok(())
    }

    #[test]
    fn read_not_found() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let handler = handlers.read(&HANDLER.id)?;

        assert!(handler.is_none());
        Ok(())
    }

    #[test]
    fn delete_ok() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        handlers.create(&HANDLER)?;
        handlers.delete(&HANDLER.id)?;
        let result = handlers.read(&HANDLER.id)?;

        assert!(result.is_none());
        Ok(())
    }

    #[test]
    #[ignore]
    fn delete_not_found() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let result = handlers.delete(&HANDLER.id);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn belonging_to_ok() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let project: Project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let mut handler = HANDLER.clone();
        handler.project_id = project.id.clone();
        handlers.create(&handler)?;

        let project_handler = handlers.belonging_to(&project)?;
        assert!(!project_handler.is_empty());
        assert_eq!(handler, *project_handler.first().unwrap());

        Ok(())
    }

    #[test]
    fn belonging_to_not_found() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let project_handlers = handlers.belonging_to(&project)?;
        assert!(project_handlers.is_empty());

        Ok(())
    }

    #[test]
    fn belonging_to_by_name_ok() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let mut handler = HANDLER.clone();
        handler.project_id = project.id.clone();
        handlers.create(&handler)?;

        let project_handler = handlers.belonging_to_by_name(&project, &HANDLER.name)?;
        assert!(project_handler.is_some());
        let project_handler = project_handler.unwrap();
        assert_eq!(handler, project_handler);
        Ok(())
    }

    #[test]
    fn belonging_to_by_name_not_found() -> anyhow::Result<()> {
        let (_temp_dir, handlers) = setup()?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());

        let project_handler = handlers.belonging_to_by_name(&project, &HANDLER.name)?;
        assert!(project_handler.is_none());
        Ok(())
    }
}
