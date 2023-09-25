use super::Command;
use crate::{config::Config, template::TemplateManager, terminal::Terminal};
use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TemplateCommand {
    /// Lists all cached templates
    List,
    /// Updates the template cache
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

    terminal.write_heading("Available templates")?;

    for (index, template) in templates.iter().enumerate() {
        let entry = format!(
            "Name:\t\t{}\nDescription:\t{}\nLanguage:\t{}",
            template.name, template.description, template.language
        );

        if index < templates.len() - 1 {
            terminal.write_text(format!("{}\n\n", entry))?;
        } else {
            terminal.write_text(format!("{}\n", entry))?;
        }
    }

    Ok(())
}
