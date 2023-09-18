use crate::{
    bindgen,
    errors::Error::{self, HandlerNotFound, ProjectNotFound},
    repository::{
        handler::{Handler, HandlerRepository},
        project::ProjectRepository,
        user::User,
        Repository,
    },
    wasmstore::WasmStore,
};
use common::{
    dtos::{GetHandlerDTO, Language},
    hash,
};

#[derive(Debug, Clone)]
pub struct HandlerService {
    projects: ProjectRepository,
    handlers: HandlerRepository,
    wasmstore: WasmStore,
}

impl HandlerService {
    pub fn new(
        projects: ProjectRepository,
        handlers: HandlerRepository,
        wasmstore: WasmStore,
    ) -> Self {
        Self {
            projects,
            handlers,
            wasmstore,
        }
    }

    pub fn create(
        &self,
        user: &User,
        project_name: &str,
        handler_name: String,
        wasm: &[u8],
    ) -> Result<(), Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;
        let old_handler = self
            .handlers
            .belonging_to_by_name(&project, &handler_name)?;

        let hash = hash::hash(wasm);
        let wasm = bindgen::create_component(wasm)?;
        // FIXME Pass correct Language
        let handler = Handler::new(handler_name, Language::Rust, hash, project.id);
        self.handlers.create(&handler)?;

        if let Some(old_handler) = old_handler {
            self.wasmstore.update(&old_handler.id, &wasm)?;
        } else {
            self.wasmstore.create(&handler.id, &wasm)?;
        }

        Ok(())
    }

    pub fn read(
        &self,
        user: &User,
        project_name: &str,
        handler_name: String,
    ) -> Result<GetHandlerDTO, Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;

        let handler = self
            .handlers
            .belonging_to_by_name(&project, &handler_name)?
            .ok_or(Error::HandlerNotFound)?;

        Ok(handler.into())
    }

    pub fn delete(&self, user: &User, project_name: &str, handler_name: &str) -> Result<(), Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;

        let handler = self
            .handlers
            .belonging_to_by_name(&project, handler_name)?
            .ok_or(HandlerNotFound)?;

        self.handlers.delete(&handler.id)?;
        self.wasmstore.delete(&handler.id)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::HandlerService;
    use crate::{
        repository::{
            handler::HandlerRepository,
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
        // FIXME: Deactivated due to the lack of the faux crate to assert a handlers has been called
    }

    #[test]
    fn create_project_not_found() {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));

        let handlers_mock = HandlerRepository::faux();
        let wasmstore_mock: WasmStore = WasmStore::faux();

        // -------------------------------------------------------------------------------------

        let handler_service = HandlerService::new(projects_mock, handlers_mock, wasmstore_mock);
        let result =
            handler_service.create(&USER, PROJECT_NAME, "handler_1".to_string(), &[0, 0, 0]);

        assert!(result.is_err())
    }

    #[test]
    #[ignore]
    fn delete_ok() {
        // FIXME: Deactivated due to the lack of the faux crate to assert a handlers has been called
    }

    #[test]
    fn delete_project_not_found() {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));

        let handlers_mock = HandlerRepository::faux();
        let wasmstore_mock: WasmStore = WasmStore::faux();

        // -------------------------------------------------------------------------------------

        let handler_service = HandlerService::new(projects_mock, handlers_mock, wasmstore_mock);
        let result = handler_service.delete(&USER, PROJECT_NAME, "handler_1");

        assert!(result.is_err())
    }

    #[test]
    fn delete_handler_not_found() {
        let handler_name = "handler_1";
        let project_expected = Project::new(PROJECT_NAME.to_string(), USER.id.clone());

        // -------------------------------------------------------------------------------------

        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(Some(project_expected.clone())));

        let mut handlers_mock = HandlerRepository::faux();
        when!(handlers_mock.belonging_to_by_name(project_expected, handler_name))
            .once()
            .then_return(Ok(None));
        let wasmstore_mock: WasmStore = WasmStore::faux();

        // -------------------------------------------------------------------------------------

        let handler_service = HandlerService::new(projects_mock, handlers_mock, wasmstore_mock);
        let result = handler_service.delete(&USER, PROJECT_NAME, handler_name);

        assert!(result.is_err())
    }
}
