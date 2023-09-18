use crate::repository::handler::{Handler, HandlerRepository};
use crate::{
    errors::Error::{self, ProjectNotFound},
    repository::{
        project::{Project, ProjectRepository},
        user::User,
        Repository,
    },
};
use common::dtos::{GetHandlerDTO, GetProjectDTO};

#[derive(Debug, Clone)]
pub struct ProjectService {
    projects: ProjectRepository,
    handlers: HandlerRepository,
}

impl ProjectService {
    pub fn new(projects: ProjectRepository, handlers: HandlerRepository) -> Self {
        Self { projects, handlers }
    }

    pub fn create(&self, user_id: String, project_name: String) -> Result<(), Error> {
        let project = Project::new(project_name, user_id);
        self.projects.create(&project)?;
        Ok(())
    }

    pub fn read(&self, user: &User, project_name: &str) -> Result<GetProjectDTO, Error> {
        let (project, handlers) = self.get_project_and_handlers(user, project_name)?;

        Ok(GetProjectDTO {
            name: project.name,
            handlers: handlers.into_iter().map(GetHandlerDTO::from).collect(),
        })
    }

    pub fn delete(&self, user: &User, project_name: &str) -> Result<(), Error> {
        let (project, handlers) = self.get_project_and_handlers(user, project_name)?;
        self.projects.delete(&project.id)?;
        for handler in handlers {
            self.handlers.delete(&handler.id)?;
        }

        Ok(())
    }

    fn get_project_and_handlers(
        &self,
        user: &User,
        project_name: &str,
    ) -> Result<(Project, Vec<Handler>), Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;
        let handlers = self.handlers.belonging_to(&project)?.into_iter().collect();

        Ok((project, handlers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user::User;
    use common::dtos::Language;
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
    fn read_ok() -> anyhow::Result<()> {
        let project_expected = Project::new(PROJECT_NAME.to_string(), USER.id.clone());

        let handler_1 = Handler::new(
            "HANDLER_1".to_string(),
            Language::Rust,
            "lohSh8xi".to_string(),
            project_expected.id.clone(),
        );
        let handler_2 = Handler::new(
            "HANDLER_2".to_string(),
            Language::Rust,
            "yie7aeH1".to_string(),
            project_expected.id.clone(),
        );
        let handlers_expected = vec![handler_1.clone(), handler_2.clone()];

        // -------------------------------------------------------------------------------------

        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(Some(project_expected.clone())));

        let mut handlers_mock = HandlerRepository::faux();
        when!(handlers_mock.belonging_to(project_expected))
            .once()
            .then_return(Ok(handlers_expected));

        // -------------------------------------------------------------------------------------

        let project_service = ProjectService::new(projects_mock, handlers_mock);
        let project = project_service.read(&USER, PROJECT_NAME)?;

        assert_eq!(PROJECT_NAME, project.name);
        assert_eq!(GetHandlerDTO::from(handler_1), project.handlers[0]);
        assert_eq!(GetHandlerDTO::from(handler_2), project.handlers[1]);

        Ok(())
    }

    #[test]
    fn read_project_not_found() -> anyhow::Result<()> {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));
        let handlers_mock = HandlerRepository::faux();

        // -------------------------------------------------------------------------------------

        let project_service = ProjectService::new(projects_mock, handlers_mock);
        let result = project_service.read(&USER, PROJECT_NAME);

        assert!(result.is_err());

        Ok(())
    }

    #[ignore]
    #[test]
    fn delete_ok() {
        // FIXME: Deactivated due to the lack of the faux crate to assert a handlers has been called
    }

    #[test]
    fn delete_project_not_found() -> anyhow::Result<()> {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));
        let handlers_mock = HandlerRepository::faux();

        // -------------------------------------------------------------------------------------

        let project_service = ProjectService::new(projects_mock, handlers_mock);
        let result = project_service.delete(&USER, PROJECT_NAME);

        assert!(result.is_err());

        Ok(())
    }
}
