use std::{fs::File, io::Read, path::Path};

use super::Command;
use crate::{config::Config, deploy, manifest::Manifest, terminal::Terminal};
use clap::Parser;
use client::{function::FunctionClient, project::ProjectClient};

#[derive(Parser, Debug)]
pub struct DeployCommand {
    pub name: Option<String>,
}

impl Command for DeployCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let manifest = Manifest::from_yaml(&config.manifest_path)?;

        let jwt = get_jwt(&config.jwt_file)?.ok_or(anyhow::anyhow!("You are not logged in"))?;

        let function_client = FunctionClient::new(&config.base_url, jwt.clone());
        let project_client = ProjectClient::new(&config.base_url, jwt);

        match self.name.clone() {
            Some(name) => deploy::deploy_function(
                &name,
                &terminal,
                manifest,
                &project_client,
                &function_client,
            )?,
            None => deploy::deploy_project(&terminal, manifest, &project_client, &function_client)?,
        }

        Ok(())
    }
}

pub fn get_jwt(path: &Path) -> anyhow::Result<Option<String>> {
    if !path.exists() {
        return Ok(None);
    }
    let mut jwt = String::default();
    let mut file = File::open(path)?;
    file.read_to_string(&mut jwt)?;
    Ok(Some(jwt))
}