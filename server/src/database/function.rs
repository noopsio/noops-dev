use super::{models::*, schema::functions, Database};
use crate::errors::Error::{self, FunctionNotFound, ProjectNotFound};
use diesel::prelude::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

impl Database {
    pub fn create_function(
        &self,
        user_id: &str,
        project_name: &str,
        function_name: String,
        wasm: &[u8],
    ) -> Result<Function, Error> {
        let project = self
            .read_project(user_id, project_name)?
            .ok_or(ProjectNotFound)?;

        let function = Function {
            id: Self::create_id(),
            name: function_name.clone(),
            hash: Self::hash(wasm),
            project_id: project.id.clone(),
        };

        let mut connection = self.pool.get().map_err(|err| anyhow::anyhow!(err))?;

        let function = if self.read_function(&project.id, &function_name)?.is_some() {
            diesel::replace_into(functions::table)
                .values(&function)
                .returning(Function::as_returning())
                .get_result(&mut connection)
                .map_err(|err| anyhow::anyhow!(err))?
        } else {
            diesel::insert_into(functions::table)
                .values(&function)
                .returning(Function::as_returning())
                .get_result(&mut connection)
                .map_err(|err| anyhow::anyhow!(err))?
        };

        Ok(function)
    }

    pub fn read_function(
        &self,
        project_id: &str,
        function_name: &str,
    ) -> anyhow::Result<Option<Function>> {
        let mut connection = self.pool.get()?;

        let function = functions::table
            .filter(functions::dsl::name.eq(function_name))
            .filter(functions::dsl::project_id.eq(project_id))
            .first::<Function>(&mut connection)
            .optional()?;

        Ok(function)
    }

    pub fn read_functions(&self, project_id: &str) -> Result<Vec<Function>, Error> {
        let mut connection = self.pool.get().map_err(|err| anyhow::anyhow!(err))?;

        let functions = functions::table
            .filter(functions::dsl::project_id.eq(project_id))
            .load::<Function>(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(functions)
    }

    pub fn delete_function(
        &self,
        user_id: &str,
        project_name: &str,
        function_name: &str,
    ) -> Result<Function, Error> {
        let project = self
            .read_project(user_id, project_name)?
            .ok_or(ProjectNotFound)?;

        self.read_function(&project.id, function_name)?
            .ok_or(FunctionNotFound)?;

        let query = functions::table
            .filter(functions::project_id.eq(project.id))
            .filter(functions::dsl::name.eq(function_name));

        let mut connection = self.pool.get().map_err(|err| anyhow::anyhow!(err))?;
        let function = diesel::delete(query)
            .get_result::<Function>(&mut connection)
            .map_err(|err| anyhow::anyhow!(err))?;
        Ok(function)
    }

    fn hash(wasm: &[u8]) -> String {
        let mut hasher = DefaultHasher::new();
        wasm.hash(&mut hasher);
        hasher.finish().to_string()
    }
}

/*

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
}

*/
