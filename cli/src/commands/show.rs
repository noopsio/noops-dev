use super::{deploy::get_jwt, Command};
use crate::{config::Config, info, manifest::Manifest, terminal::Terminal};
use clap::Parser;
use client::{function::FunctionClient, project::ProjectClient};

#[derive(Parser, Debug)]
pub struct ShowCommand {
    pub name: Option<String>,
}

impl Command for ShowCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let manifest = Manifest::from_yaml(&config.manifest)?;

        let jwt = get_jwt(&config.jwt_file)?.ok_or(anyhow::anyhow!("You are not logged in"))?;
        let function_client = FunctionClient::new(&config.base_url, jwt.clone());
        let project_client = ProjectClient::new(&config.base_url, jwt);

        match self.name.clone() {
            Some(name) => info::show_function(&name, &manifest, &function_client, &terminal)?,
            None => info::show_project(&manifest, &project_client, &terminal)?,
        }

        Ok(())
    }
}
