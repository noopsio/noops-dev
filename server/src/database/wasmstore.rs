use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::errors::Error::{
    self, FunctionNotFound, ProjectAlreadyExists, ProjectNotFound, UserAlreadyExists, UserNotFound,
};

#[derive(Debug, Clone)]
pub struct WasmStore {
    prefix: PathBuf,
}

impl WasmStore {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        fs::create_dir_all(path)?;
        Ok(Self {
            prefix: path.to_path_buf(),
        })
    }

    pub fn create_user(&self, user: &str) -> Result<(), Error> {
        let user_path = self.prefix.join(user);
        if user_path.exists() {
            return Err(UserAlreadyExists);
        }

        fs::create_dir(user_path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn _delete_user(&self, user: &str) -> Result<(), Error> {
        let user_path = self.prefix.join(user);
        if !user_path.exists() {
            return Err(UserNotFound);
        }
        fs::remove_dir_all(user_path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn create_project(&self, user: &str, project: &str) -> Result<(), Error> {
        let user_path = self.prefix.join(user);
        if !user_path.exists() {
            return Err(UserNotFound);
        }

        let project_path = user_path.join(project);
        if project_path.exists() {
            return Err(ProjectAlreadyExists);
        }

        fs::create_dir(project_path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn delete_project(&self, user: &str, project: &str) -> Result<(), Error> {
        let user_path = self.prefix.join(user);
        if !user_path.exists() {
            return Err(UserNotFound);
        }

        let project_path = user_path.join(project);
        if !project_path.exists() {
            return Err(ProjectNotFound);
        }

        fs::remove_dir_all(project_path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn create_function(
        &self,
        user: &str,
        project: &str,
        function: &str,
        wasm: &[u8],
    ) -> Result<(), Error> {
        let user_path = self.prefix.join(user);
        if !user_path.exists() {
            return Err(UserNotFound);
        }

        let project_path = user_path.join(project);
        if !project_path.exists() {
            return Err(ProjectNotFound);
        }

        let function_path = project_path.join(format!("{}.wasm", function));

        let mut file = File::create(function_path).map_err(|err| anyhow::anyhow!(err))?;
        file.write_all(wasm).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn delete_function(&self, user: &str, project: &str, function: &str) -> Result<(), Error> {
        let user_path = self.prefix.join(user);
        if !user_path.exists() {
            return Err(UserNotFound);
        }

        let project_path = user_path.join(project);
        if !project_path.exists() {
            return Err(ProjectNotFound);
        }

        let function_path = project_path.join(format!("{}.wasm", function));
        if !project_path.exists() {
            return Err(FunctionNotFound);
        }

        fs::remove_file(function_path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn read_function(
        &self,
        user: &str,
        project: &str,
        function: &str,
    ) -> Result<Vec<u8>, Error> {
        let user_path = self.prefix.join(user);
        if !user_path.exists() {
            return Err(UserNotFound);
        }

        let project_path = user_path.join(project);
        if !project_path.exists() {
            return Err(ProjectNotFound);
        }

        let function_path = project_path.join(format!("{}.wasm", function));
        if !project_path.exists() {
            return Err(FunctionNotFound);
        }

        let wasm = fs::read(function_path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(wasm)
    }
}
