use super::{
    create_id,
    project::Project,
    schema::functions::{self, dsl},
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
#[diesel(table_name = crate::repository::schema::functions)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Function {
    pub id: String,
    pub name: String,
    pub language: Language,
    pub hash: String,
    pub project_id: String,
}

impl Function {
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
pub struct FunctionRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[cfg_attr(test, faux::methods)]
impl Repository<Function> for FunctionRepository {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    fn create(&self, function: &Function) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::insert_into(functions::table)
            .values(function)
            .on_conflict((dsl::name, dsl::project_id))
            .do_update()
            .set(function)
            .execute(&mut connection)?;

        Ok(())
    }

    fn read(&self, id: &str) -> anyhow::Result<Option<Function>> {
        let mut connection = self.pool.get()?;

        let function = functions::table
            .find(id)
            .first::<Function>(&mut connection)
            .optional()?;

        Ok(function)
    }

    fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::delete(functions::table.find(id))
            .execute(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(())
    }
}

#[cfg_attr(test, faux::methods)]
impl FunctionRepository {
    pub fn belonging_to(&self, project: &Project) -> anyhow::Result<Vec<Function>> {
        let mut connection = self.pool.get()?;
        let functions = Function::belonging_to(project).load::<Function>(&mut connection)?;
        Ok(functions)
    }

    pub fn belonging_to_by_name(
        &self,
        project: &Project,
        function_name: &str,
    ) -> anyhow::Result<Option<Function>> {
        let mut connection = self.pool.get()?;

        let function = Function::belonging_to(project)
            .filter(dsl::name.eq(function_name))
            .first::<Function>(&mut connection)
            .optional()?;

        Ok(function)
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
    const FUNCTION_NAME: &str = "FUNCTION_NAME";
    const FUNCTION_HASH: &str = "aeHae9shaiquu4Iey8phishohqu1ON9y";
    const PROJECT_ID: &str = "xiekaiphoe7Luk3zeuNie";
    const PROJECT_NAME: &str = "PROJECT_NAME";
    const USER_ID: &str = "puphoonoh1bae6Binaixu";
    const FUNCTION_LANGUAGE: Language = Language::Rust;

    lazy_static! {
        static ref FUNCTION: Function = Function::new(
            FUNCTION_NAME.to_string(),
            FUNCTION_LANGUAGE,
            FUNCTION_HASH.to_string(),
            PROJECT_ID.to_string()
        );
    }

    fn setup() -> anyhow::Result<(TempDir, FunctionRepository)> {
        let temp_dir = tempdir()?;
        let pool = create_pool(&temp_dir.path().join(DATABASE_NAME));
        let mut connection = pool.get()?;
        let projects = FunctionRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        connection.run_pending_migrations(migrations).unwrap();
        Ok((temp_dir, projects))
    }

    #[test]
    fn create_ok() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let result = functions.create(&FUNCTION);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn create_conflict() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        functions.create(&FUNCTION)?;
        let result = functions.create(&FUNCTION);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn create_name_conflict() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        functions.create(&FUNCTION)?;
        let function = Function::new(
            FUNCTION_NAME.to_string(),
            FUNCTION_LANGUAGE,
            FUNCTION_HASH.to_string(),
            PROJECT_ID.to_string(),
        );
        let result = functions.create(&function);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn read_ok() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        functions.create(&FUNCTION)?;
        let function = functions.read(&FUNCTION.id)?;

        assert!(function.is_some());
        let function = function.unwrap();
        assert_eq!(*FUNCTION, function);
        Ok(())
    }

    #[test]
    fn read_not_found() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let function = functions.read(&FUNCTION.id)?;

        assert!(function.is_none());
        Ok(())
    }

    #[test]
    fn delete_ok() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        functions.create(&FUNCTION)?;
        functions.delete(&FUNCTION.id)?;
        let result = functions.read(&FUNCTION.id)?;

        assert!(result.is_none());
        Ok(())
    }

    #[test]
    #[ignore]
    fn delete_not_found() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let result = functions.delete(&FUNCTION.id);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn belonging_to_ok() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let project: Project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let mut function = FUNCTION.clone();
        function.project_id = project.id.clone();
        functions.create(&function)?;

        let project_functions = functions.belonging_to(&project)?;
        assert!(!project_functions.is_empty());
        assert_eq!(function, *project_functions.first().unwrap());

        Ok(())
    }

    #[test]
    fn belonging_to_not_found() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let project_functions = functions.belonging_to(&project)?;
        assert!(project_functions.is_empty());

        Ok(())
    }

    #[test]
    fn belonging_to_by_name_ok() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());
        let mut function = FUNCTION.clone();
        function.project_id = project.id.clone();
        functions.create(&function)?;

        let project_function = functions.belonging_to_by_name(&project, &FUNCTION.name)?;
        assert!(project_function.is_some());
        let project_function = project_function.unwrap();
        assert_eq!(function, project_function);
        Ok(())
    }

    #[test]
    fn belonging_to_by_name_not_found() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        let project = Project::new(PROJECT_NAME.to_string(), USER_ID.to_string());

        let project_function = functions.belonging_to_by_name(&project, &FUNCTION.name)?;
        assert!(project_function.is_none());
        Ok(())
    }
}
