use super::Command;
use crate::{config::Config, manifest::Manifest, terminal::Terminal};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ShowCommand {
    pub name: Option<String>,
}

impl Command for ShowCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let manifest = Manifest::from_yaml(&config.manifest_path)?;

        match self.name.clone() {
            Some(name) => show_function()?,
            None => show_project()?,
        }

        Ok(())
    }
}

fn show_project(manifest: &Manifest) -> anyhow::Result<()> {
    todo!()
}

fn show_function() -> anyhow::Result<()> {
    todo!()
}
