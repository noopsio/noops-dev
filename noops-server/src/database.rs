use anyhow;
use jammdb::{Data, DB};
use std::path::Path;

use crate::schemas;

const PROJECT_BUCKET: &str = "PROJECTS";

pub struct Database {
    database: DB,
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Database> {
        let database = DB::open(path)?;
        let tx = database.tx(true)?;

        if tx.buckets().count() == 0 {
            tx.create_bucket(PROJECT_BUCKET)?;
        }

        tx.commit()?;
        Ok(Database { database })
    }

    pub fn project_exists(&self, project_name: &str) -> anyhow::Result<bool> {
        let tx = self.database.tx(false)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        match root.get_bucket(project_name) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn project_create(&self, project_name: &str) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        let _ = root.create_bucket(project_name)?;
        tx.commit()?;
        Ok(())
    }

    pub fn project_delete(&self, project_name: &str) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        let _ = root.delete_bucket(project_name)?;
        tx.commit()?;
        Ok(())
    }

    pub fn project_list(
        &self,
        project_name: &str,
    ) -> anyhow::Result<Vec<schemas::GetFunctionSchema>> {
        let tx = self.database.tx(false)?;
        let bucket = tx.get_bucket(PROJECT_BUCKET)?;
        let projects = bucket.get_bucket(project_name)?;

        let mut functions = Vec::new();
        for data in projects.cursor() {
            if let Data::KeyValue(kv) = data {
                let function: schemas::CreateFunctionSchema = bincode::deserialize(kv.value())?;
                functions.push(schemas::GetFunctionSchema {
                    name: function.name,
                    project: function.project,
                    params: function.params,
                });
            }
        }
        Ok(functions)
    }

    pub fn function_exists(&self, project_name: &str, function_name: &str) -> anyhow::Result<bool> {
        let tx = self.database.tx(false)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        return if let Ok(project) = root.get_bucket(project_name) {
            return if let Some(_) = project.get(function_name) {
                Ok(true)
            } else {
                Ok(false)
            };
        } else {
            Ok(false)
        };
    }

    pub fn function_create(
        &self,
        project_name: &str,
        function_name: &str,
        function: &schemas::CreateFunctionSchema,
    ) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let bucket = tx.get_bucket(PROJECT_BUCKET)?;
        let project = bucket.get_bucket(project_name)?;

        project.put(function_name, bincode::serialize(&function)?)?;
        tx.commit()?;

        Ok(())
    }

    pub fn function_get(
        &self,
        project_name: &str,
        function_name: &str,
    ) -> anyhow::Result<schemas::CreateFunctionSchema> {
        let tx = self.database.tx(false)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        let projects = root.get_bucket(project_name)?;
        let kv = projects.get_kv(function_name).unwrap();
        let data = kv.value();
        Ok(bincode::deserialize(data)?)
    }

    pub fn function_delete(&self, project_name: &str, function_name: &str) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        let project = root.get_bucket(project_name)?;

        project.delete(function_name)?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::schemas::CreateFunctionSchema;

    use super::*;
    use lazy_static::lazy_static;
    use tempfile::tempdir;

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref FUNCTION_SCHEMA: CreateFunctionSchema = CreateFunctionSchema {
            project: PROJECT_NAME.to_string(),
            name: FUNCTION_NAME.to_string(),
            wasm: Default::default(),
            params: Default::default(),
        };
    }

    #[test]
    fn new() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();
        let tx = db.database.tx(false).unwrap();
        let _ = tx.get_bucket(PROJECT_BUCKET).unwrap();
    }

    #[test]
    fn project_create() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();

        db.project_create(PROJECT_NAME).unwrap();
        assert!(db.project_exists(PROJECT_NAME).unwrap());
    }

    #[test]
    fn project_delete() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();

        db.project_create(PROJECT_NAME).unwrap();
        assert!(db.project_exists(PROJECT_NAME).unwrap());

        db.project_delete(PROJECT_NAME).unwrap();
        assert!(!db.project_exists(PROJECT_NAME).unwrap());
    }

    #[test]
    fn function_create() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();

        db.project_create(PROJECT_NAME).unwrap();
        db.function_create(PROJECT_NAME, FUNCTION_NAME, &FUNCTION_SCHEMA)
            .unwrap();
        assert!(db.function_exists(PROJECT_NAME, FUNCTION_NAME).unwrap());

        let function = db.function_get(PROJECT_NAME, FUNCTION_NAME).unwrap();
        assert_eq!(*FUNCTION_SCHEMA, function);
    }

    #[test]
    fn function_delete() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();

        db.project_create(PROJECT_NAME).unwrap();
        db.function_create(PROJECT_NAME, FUNCTION_NAME, &FUNCTION_SCHEMA)
            .unwrap();
        assert!(db.function_exists(PROJECT_NAME, FUNCTION_NAME).unwrap());

        db.function_delete(PROJECT_NAME, FUNCTION_NAME).unwrap();
        assert!(!db.function_exists(PROJECT_NAME, FUNCTION_NAME).unwrap());
    }
}
