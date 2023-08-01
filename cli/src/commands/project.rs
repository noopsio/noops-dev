use super::Command;
use crate::{
    client::NoopsClient,
    config::Config,
    handlers,
    manifest::{self, Manifest},
    terminal::Terminal,
};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Create a new project
    Create(ProjectCreateCommand),
    /// Build the project
    Build(ProjectBuildCommand),
    /// Deploy the function
    Deploy(ProjectDeployCommand),
    /// Destroy the project
    Destroy(ProjectDestroyCommand),
    /// Show information about the project
    Show,
}

#[derive(Parser, Debug)]
pub struct ProjectCreateCommand;

impl Command for ProjectCreateCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        handlers::project::init(&terminal)?;
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct ProjectBuildCommand;

impl Command for ProjectBuildCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let manifest = Manifest::from_yaml(manifest::MANIFEST_FILE_NAME)?;
        handlers::project::build(&terminal, &manifest)?;
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct ProjectDeployCommand;

impl Command for ProjectDeployCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let manifest = Manifest::from_yaml(manifest::MANIFEST_FILE_NAME)?;
        let config = Config::default();

        let jwt = handlers::auth::get_jwt(&config.jwt_file)?;
        if jwt.is_none() {
            terminal.write_text("You are not logged in.")?;
            return Ok(());
        }
        let client = NoopsClient::new(config.base_url, manifest.project_name.clone(), jwt);
        handlers::project::deploy(&terminal, &manifest, &client)?;
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct ProjectDestroyCommand;

impl Command for ProjectDestroyCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let manifest = Manifest::from_yaml(manifest::MANIFEST_FILE_NAME)?;
        let config = Config::default();

        let jwt = handlers::auth::get_jwt(&config.jwt_file)?;
        if jwt.is_none() {
            terminal.write_text("You are not logged in.")?;
            return Ok(());
        }
        let client = NoopsClient::new(config.base_url, manifest.project_name.clone(), jwt);
        handlers::project::destroy(&terminal, &client, &manifest.project_name)?;

        Ok(())
    }
}
