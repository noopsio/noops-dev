use crate::repository::function::{Function, FunctionRepository};
use crate::{
    errors::Error::{self, ProjectNotFound},
    repository::{
        project::{Project, ProjectRepository},
        user::User,
        Repository,
    },
};
use dtos::{GetFunctionDTO, GetProjectDTO};

#[derive(Debug, Clone)]
pub struct ProjectService {
    projects: ProjectRepository,
    functions: FunctionRepository,
}

impl ProjectService {
    pub fn new(projects: ProjectRepository, functions: FunctionRepository) -> Self {
        Self {
            projects,
            functions,
        }
    }

    pub fn create(&self, user_id: String, project_name: String) -> Result<(), Error> {
        let project = Project::new(project_name, user_id);
        self.projects.create(&project)?;
        Ok(())
    }

    pub fn read(&self, user: &User, project_name: &str) -> Result<GetProjectDTO, Error> {
        let (project, functions) = self.get_project_and_functions(user, project_name)?;

        Ok(GetProjectDTO {
            name: project.name,
            functions: functions.into_iter().map(GetFunctionDTO::from).collect(),
        })
    }

    pub fn delete(&self, user: &User, project_name: &str) -> Result<(), Error> {
        let (project, functions) = self.get_project_and_functions(user, project_name)?;
        self.projects.delete(&project.id)?;
        for function in functions {
            self.functions.delete(&function.id)?;
        }

        Ok(())
    }

    fn get_project_and_functions(
        &self,
        user: &User,
        project_name: &str,
    ) -> Result<(Project, Vec<Function>), Error> {
        let project = self
            .projects
            .belonging_to_by_name(user, project_name)?
            .ok_or(ProjectNotFound)?;
        let functions = self.functions.belonging_to(&project)?.into_iter().collect();

        Ok((project, functions))
    }
}
<<<<<<< HEAD
=======

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user::User;
    use faux::when;
    use lazy_static::lazy_static;

    const PROJECT_NAME: &str = "PROJECT_NAME";
    const USER_EMAIL: &str = "test@example.com";
    const USER_GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref USER: User = User::new(
            USER_EMAIL.to_string(),
            USER_GH_ID,
            USER_GH_ACCESS_TOKEN.to_string(),
        );
    }

    #[test]
    fn read_ok() -> anyhow::Result<()> {
        let project_expected = Project::new(PROJECT_NAME.to_string(), USER.id.clone());

        let function_1 = Function::new(
            "FUNCTION_1".to_string(),
            "lohSh8xi".to_string(),
            project_expected.id.clone(),
        );
        let function_2 = Function::new(
            "FUNCTION_2".to_string(),
            "yie7aeH1".to_string(),
            project_expected.id.clone(),
        );
        let functions_expected = vec![function_1.clone(), function_2.clone()];

        // -------------------------------------------------------------------------------------

        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(Some(project_expected.clone())));

        let mut functions_mock = FunctionRepository::faux();
        when!(functions_mock.belonging_to(project_expected))
            .once()
            .then_return(Ok(functions_expected));

        // -------------------------------------------------------------------------------------

        let project_service = ProjectService::new(projects_mock, functions_mock);
        let project = project_service.read(&USER, PROJECT_NAME)?;

        assert_eq!(PROJECT_NAME, project.name);
        assert_eq!(GetFunctionDTO::from(function_1), project.functions[0]);
        assert_eq!(GetFunctionDTO::from(function_2), project.functions[1]);

        Ok(())
    }

    #[test]
    fn read_project_not_found() -> anyhow::Result<()> {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));
        let functions_mock = FunctionRepository::faux();

        // -------------------------------------------------------------------------------------

        let project_service = ProjectService::new(projects_mock, functions_mock);
        let result = project_service.read(&USER, PROJECT_NAME);

        assert!(result.is_err());

        Ok(())
    }

    #[ignore]
    #[test]
    fn delete_ok() {
        // FIXME: Deactivated due to the lack of the faux crate to assert a functions has been called
    }

    #[test]
    fn delete_project_not_found() -> anyhow::Result<()> {
        let mut projects_mock = ProjectRepository::faux();
        when!(projects_mock.belonging_to_by_name(USER.clone(), PROJECT_NAME))
            .once()
            .then_return(Ok(None));
        let functions_mock = FunctionRepository::faux();

        // -------------------------------------------------------------------------------------

        let project_service = ProjectService::new(projects_mock, functions_mock);
        let result = project_service.delete(&USER, PROJECT_NAME);

        assert!(result.is_err());

        Ok(())
    }
}
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
