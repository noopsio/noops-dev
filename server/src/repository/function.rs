use super::{create_id, project::Project, schema::functions, Repository};
use anyhow;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dtos::GetFunctionDTO;

#[derive(
    Identifiable, Insertable, Queryable, Selectable, Associations, Debug, Clone, PartialEq,
)]
#[diesel(table_name = crate::repository::schema::functions)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Function {
    pub id: String,
    pub name: String,
    pub hash: String,
    pub project_id: String,
}

impl Function {
    pub fn new(name: String, hash: String, project_id: String) -> Self {
        Self {
            id: create_id(),
            name,
            hash,
            project_id,
        }
    }
}

impl From<Function> for GetFunctionDTO {
    fn from(function: Function) -> Self {
        GetFunctionDTO {
            name: function.name,
            hash: function.hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Repository<Function> for FunctionRepository {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    fn create(&self, function: &Function) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        diesel::insert_into(functions::table)
            .values(function)
            .execute(&mut connection)?;

        Ok(())
    }

    fn read(&self, id: &str) -> anyhow::Result<Option<Function>> {
        let mut connection = self.pool.get()?;

        let function = functions::dsl::functions
            .find(id)
            .first::<Function>(&mut connection)
            .optional()?;

        Ok(function)
    }

    fn delete(&self, id: &str) -> anyhow::Result<Function> {
        let mut connection = self.pool.get()?;

        let function = diesel::delete(functions::table.find(id))
            .get_result::<Function>(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(function)
    }
}

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
            .filter(functions::dsl::name.eq(function_name))
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

    lazy_static! {
        static ref FUNCTION: Function = Function::new(
            FUNCTION_NAME.to_string(),
            FUNCTION_HASH.to_string(),
            PROJECT_ID.to_string()
        );
    }

    fn setup() -> anyhow::Result<(TempDir, FunctionRepository)> {
        let temp_dir = tempdir()?;
        let pool = create_pool(&temp_dir.path().join(DATABASE_NAME));
        let projects = FunctionRepository::new(pool);
        let migrations = FileBasedMigrations::find_migrations_directory_in_path("./server")?;
        let mut connection = projects.pool.get()?;
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
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn create_name_conflict() -> anyhow::Result<()> {
        let (_temp_dir, functions) = setup()?;
        functions.create(&FUNCTION)?;
        let function = Function::new(
            FUNCTION_NAME.to_string(),
            FUNCTION_HASH.to_string(),
            PROJECT_ID.to_string(),
        );
        let result = functions.create(&function);
        assert!(result.is_err());
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
        let deleted_function = functions.delete(&FUNCTION.id)?;

        assert_eq!(*FUNCTION, deleted_function);
        Ok(())
    }

    #[test]
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
