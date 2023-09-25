use super::Command;
use crate::{build, config::Config, manifest::Manifest, terminal::Terminal};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct BuildCommand {
    /// The handler to build
    pub name: Option<String>,
}

impl Command for BuildCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let manifest = Manifest::from_yaml(&config.manifest)?;

        match self.name.clone() {
            Some(name) => build::build_handler(&terminal, &manifest, &name)?,
            None => build::build_project(&terminal, &manifest)?,
        }

        Ok(())
    }
}
