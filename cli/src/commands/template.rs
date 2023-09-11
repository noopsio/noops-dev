use super::Command;
use crate::{config::Config, template::TemplateManager, terminal::Terminal};
use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TemplateCommand {
    List,
    Update,
}

impl Command for TemplateCommand {
    fn execute(&self) -> anyhow::Result<()> {
        match &self {
            TemplateCommand::List => list(),
            TemplateCommand::Update => update(),
        }
    }
}

fn update() -> Result<()> {
    let config = Config::default();
    let manager = TemplateManager::new();
    manager.update(&config.templates_dir)?;
    Ok(())
}

fn list() -> Result<()> {
    let config = Config::default();
    let manager = TemplateManager::new();
    let terminal = Terminal::new();
    let templates = manager.list(&config.template_manifest)?;
    terminal.write_text(format!("{:?}", templates))?;
    Ok(())
}
