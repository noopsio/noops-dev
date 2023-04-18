use anyhow;
use jammdb::{Data, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;

const PROJECT_BUCKET: &str = "PROJECTS";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub project: String,
    pub component: Vec<u8>,
    pub hash: u64,
}

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
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        match projects.get_bucket(project_name) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn project_create(&self, project_name: &str) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        let _ = projects.create_bucket(project_name)?;
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

    pub fn project_get(&self, project_name: &str) -> anyhow::Result<Vec<Function>> {
        let tx = self.database.tx(false)?;
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        let all_functions = projects.get_bucket(project_name)?;

        let mut functions: Vec<Function> = Vec::new();
        for data in all_functions.into_iter() {
            if let Data::KeyValue(kv) = data {
                let function: Function = bincode::deserialize(kv.value())?;
                functions.push(function);
            }
        }
        Ok(functions)
    }

    pub fn function_exists(&self, project_name: &str, function_name: &str) -> anyhow::Result<bool> {
        let tx = self.database.tx(false)?;
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        return if let Ok(project) = projects.get_bucket(project_name) {
            return if let Some(_) = project.get(function_name) {
                Ok(true)
            } else {
                Ok(false)
            };
        } else {
            Ok(false)
        };
    }

    pub fn function_create(&self, function: &Function) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        let project = projects.get_bucket(function.project.clone())?;
        project.put(function.name.clone(), bincode::serialize(&function)?)?;
        tx.commit()?;
        Ok(())
    }

    pub fn function_get(
        &self,
        project_name: &str,
        function_name: &str,
    ) -> anyhow::Result<Function> {
        let tx = self.database.tx(false)?;
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        let functions = projects.get_bucket(project_name)?;
        let kv = functions.get_kv(function_name).unwrap();
        let data = kv.value();
        Ok(bincode::deserialize(data)?)
    }

    pub fn function_delete(&self, project_name: &str, function_name: &str) -> anyhow::Result<()> {
        let tx = self.database.tx(true)?;
        let projects = tx.get_bucket(PROJECT_BUCKET)?;
        let functions = projects.get_bucket(project_name)?;

        functions.delete(function_name)?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use lazy_static::lazy_static;
    use tempfile::tempdir;

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref FUNCTION: Function = Function {
            name: FUNCTION_NAME.to_string(),
            project: PROJECT_NAME.to_string(),
            ..Default::default()
        };
    }

    #[test]
    fn new() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let db = Database::new(temp_dir.path().join(DATABASE_NAME))?;
        let tx = db.database.tx(false)?;
        let _ = tx.get_bucket(PROJECT_BUCKET)?;
        Ok(())
    }

    #[test]
    fn project_create() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let db = Database::new(temp_dir.path().join(DATABASE_NAME))?;

        db.project_create(PROJECT_NAME)?;
        assert!(db.project_exists(PROJECT_NAME)?);
        Ok(())
    }

    #[test]
    fn project_delete() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let db = Database::new(temp_dir.path().join(DATABASE_NAME))?;

        db.project_create(PROJECT_NAME)?;
        assert!(db.project_exists(PROJECT_NAME)?);

        db.project_delete(PROJECT_NAME)?;
        assert!(!db.project_exists(PROJECT_NAME)?);
        Ok(())
    }

    #[test]
    fn project_get() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let db = Database::new(temp_dir.path().join(DATABASE_NAME))?;

        db.project_create(PROJECT_NAME)?;
        db.function_create(&FUNCTION)?;
        let functions = db.project_get(PROJECT_NAME)?;
        assert_eq!(vec![FUNCTION.clone()], functions);
        Ok(())
    }

    #[test]
    fn function_create() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let db = Database::new(temp_dir.path().join(DATABASE_NAME))?;

        db.project_create(PROJECT_NAME)?;
        db.function_create(&FUNCTION)?;
        assert!(db.function_exists(PROJECT_NAME, FUNCTION_NAME)?);
        Ok(())
    }

    #[test]
    fn function_delete() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let db = Database::new(temp_dir.path().join(DATABASE_NAME))?;

        db.project_create(PROJECT_NAME)?;
        db.function_create(&FUNCTION)?;
        assert!(db.function_exists(PROJECT_NAME, FUNCTION_NAME)?);

        db.function_delete(PROJECT_NAME, FUNCTION_NAME)?;
        assert!(!db.function_exists(PROJECT_NAME, FUNCTION_NAME)?);
        Ok(())
    }
}
