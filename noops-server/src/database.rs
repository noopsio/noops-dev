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
        let tx = self.database.tx(true)?;
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
    ) -> anyhow::Result<Vec<schemas::CreateFunctionSchema>> {
        let tx = self.database.tx(false)?;
        let bucket = tx.get_bucket(PROJECT_BUCKET)?;
        let projects = bucket.get_bucket(project_name)?;

        let mut functions = Vec::new();
        for data in projects.cursor() {
            if let Data::KeyValue(kv) = data {
                let function: schemas::CreateFunctionSchema = bincode::deserialize(kv.value())?;
                functions.push(function);
            }
        }
        Ok(functions)
    }

    pub fn function_exists(&self, project_name: &str, function_name: &str) -> anyhow::Result<bool> {
        let tx = self.database.tx(true)?;
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
        let projects = bucket.get_bucket(project_name)?;

        projects.put(function_name, bincode::serialize(&function)?)?;
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
        let tx = self.database.tx(false)?;
        let root = tx.get_bucket(PROJECT_BUCKET)?;
        let projects = root.get_bucket(project_name)?;

        projects.delete(function_name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::tempdir;

    static DATABASE_NAME: &str = "noops.test_db";
    static PROJECT_NAME: &str = "test_project";
    static FUNCTION_NAME: &str = "test_function";

    #[test]
    fn new() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();
        let tx = db.database.tx(false).unwrap();
        let _ = tx.get_bucket(PROJECT_BUCKET).unwrap();
    }

    #[test]
    fn create_project() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();
        db.project_create(PROJECT_NAME).unwrap();
        let functions = db.project_list(PROJECT_NAME).unwrap();
        assert!(functions.is_empty());
    }

    #[test]
    fn create_function() {
        let temp_dir = tempdir().unwrap();
        let db = Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap();
        db.project_create(PROJECT_NAME).unwrap();

        let test_function = schemas::CreateFunctionSchema {
            project: PROJECT_NAME.to_string(),
            name: FUNCTION_NAME.to_string(),
            params: vec!["param1".to_string(), "param2".to_string()],
            wasm: vec![0, 0, 0, 0, 0, 0, 0],
        };

        db.function_create(PROJECT_NAME, FUNCTION_NAME, &test_function)
            .unwrap();
        let function = db.function_get(PROJECT_NAME, FUNCTION_NAME).unwrap();
        assert_eq!(test_function, function);
    }
}
