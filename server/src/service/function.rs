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
use common::{
    dtos::{GetFunctionDTO, Language},
    hash,
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
        let old_function = self
            .functions
            .belonging_to_by_name(&project, &function_name)?;

        let hash = hash::hash(wasm);
        let wasm = bindgen::create_component(wasm)?;
        // FIXME Pass correct Language
        let function = Function::new(function_name, Language::Rust, hash, project.id);
        self.functions.create(&function)?;

        if let Some(old_function) = old_function {
            self.wasmstore.update(&old_function.id, &wasm)?;
        } else {
            self.wasmstore.create(&function.id, &wasm)?;
        }

        Ok(())
    }

    pub fn read(
        &self,
        user: &User,
        project_name: &str,
        function_name: String,
    ) -> Result<GetFunctionDTO, Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;

        let function = self
            .functions
            .belonging_to_by_name(&project, &function_name)?
            .ok_or(Error::FunctionNotFound)?;

        Ok(function.into())
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

#[cfg(test)]
mod tests {
    use super::FunctionService;
    use crate::{
        repository::{
            function::FunctionRepository,
            project::{Project, ProjectRepository},
            user::User,
        },
        wasmstore::WasmStore,
    };
    use faux::when;
    use lazy_static::lazy_static;

    const PROJECT_NAME: &str = "PROJECT_NAME";

    const USER_EMAIL: &str = "test@example.com";
    const USER_NAME: &str = "user_name";
    const USER_LOCATION: &str = "Hamburg";
    const USER_COMPANY: &str = "Noops.io";
    const USER_GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const USER_GH_LOGIN: &str = "login_name";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref USER: User = User::new(
            USER_EMAIL.to_string(),
            Some(USER_NAME.to_string()),
            Some(USER_LOCATION.to_string()),
            Some(USER_COMPANY.to_string()),
            USER_GH_ID,
            USER_GH_LOGIN.to_string(),
            USER_GH_ACCESS_TOKEN.to_string()
        );
    }

    #[test]
    #[ignore]
    fn create_ok() {
        // FIXME: Deactivated due to the lack of the faux crate to assert a functions has been called
    }

    #[test]
    fn create_project_not_found() {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));

        let functions_mock = FunctionRepository::faux();
        let wasmstore_mock: WasmStore = WasmStore::faux();

        // -------------------------------------------------------------------------------------

        let function_service = FunctionService::new(projects_mock, functions_mock, wasmstore_mock);
        let result =
            function_service.create(&USER, PROJECT_NAME, "function_1".to_string(), &[0, 0, 0]);

        assert!(result.is_err())
    }

    #[test]
    #[ignore]
    fn delete_ok() {
        // FIXME: Deactivated due to the lack of the faux crate to assert a functions has been called
    }

    #[test]
    fn delete_project_not_found() {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));

        let functions_mock = FunctionRepository::faux();
        let wasmstore_mock: WasmStore = WasmStore::faux();

        // -------------------------------------------------------------------------------------

        let function_service = FunctionService::new(projects_mock, functions_mock, wasmstore_mock);
        let result = function_service.delete(&USER, PROJECT_NAME, "function_1");

        assert!(result.is_err())
    }

    #[test]
    fn delete_function_not_found() {
        let function_name = "function_1";
        let project_expected = Project::new(PROJECT_NAME.to_string(), USER.id.clone());

        // -------------------------------------------------------------------------------------

        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(Some(project_expected.clone())));

        let mut functions_mock = FunctionRepository::faux();
        when!(functions_mock.belonging_to_by_name(project_expected, function_name))
            .once()
            .then_return(Ok(None));
        let wasmstore_mock: WasmStore = WasmStore::faux();

        // -------------------------------------------------------------------------------------

        let function_service = FunctionService::new(projects_mock, functions_mock, wasmstore_mock);
        let result = function_service.delete(&USER, PROJECT_NAME, function_name);

        assert!(result.is_err())
    }
}
