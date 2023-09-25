use super::Command;
use crate::{config::Config, manifest::Manifest, terminal::Terminal};
use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct InitCommand {
    /// The name of the project
    pub name: String,
}

impl Command for InitCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();

        Manifest::init(&self.name, &config.manifest).context("Initializing project failed")?;
        terminal.write_text(format!("{} successfully initialized", &self.name))?;
        Ok(())
    }
}
