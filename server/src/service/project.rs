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
