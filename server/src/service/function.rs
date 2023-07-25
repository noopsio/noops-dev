use crate::{
    bindgen,
    errors::Error::{self, FunctionNotFound, ProjectNotFound},
    repository::{
        function::{Function, FunctionRepository},
        project::ProjectRepository,
        user::User,
        Repository,
    },
    wasmstore::WasmStore,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
pub struct FunctionService {
    projects: ProjectRepository,
    functions: FunctionRepository,
    wasmstore: WasmStore,
}

impl FunctionService {
    pub fn new(
        projects: ProjectRepository,
        functions: FunctionRepository,
        wasmstore: WasmStore,
    ) -> Self {
        Self {
            projects,
            functions,
            wasmstore,
        }
    }

    pub fn create(
        &self,
        user: &User,
        project_name: &str,
        function_name: String,
        wasm: &[u8],
    ) -> Result<(), Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;
        let wasm = bindgen::create_component(wasm)?;
        let hash = Self::hash(&wasm);
        let function = Function::new(function_name, hash, project.id);
        self.functions.create(&function)?;
        self.wasmstore.create(&function.id, &wasm)?;

        Ok(())
    }

    fn hash(wasm: &[u8]) -> String {
        let mut hasher = DefaultHasher::new();
        wasm.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn delete(
        &self,
        user: &User,
        project_name: &str,
        function_name: &str,
    ) -> Result<(), Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;

        let function = self
            .functions
            .belonging_to_by_name(&project, function_name)?
            .ok_or(FunctionNotFound)?;

        self.functions.delete(&function.id)?;
        self.wasmstore.delete(&function.id)?;

        Ok(())
    }
}
