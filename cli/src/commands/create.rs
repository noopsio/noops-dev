use super::Command;
use crate::{
    config::Config,
    manifest::{Handler, Manifest},
    template::{Template, TemplateManager},
    terminal::Terminal,
};
use anyhow::{bail, Context};
use clap::Parser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
pub struct CreateCommand {
    /// The name of the new handler
    pub name: String,
}

impl Command for CreateCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let template_manager = TemplateManager::new();
        let mut manifest = Manifest::from_yaml(&config.manifest)?;

        terminal.write_heading("Creating a handler")?;

        if !config.template_manifest.exists() {
            bail!("Templates not synced - Use \"noops template update\"");
        }

        let templates = template_manager.list(&config.template_manifest)?;

        let index = terminal.select_prompt("Select a template", &templates)?;
        let mut template = templates[index].clone();
        template.name = self.name.clone();

        let text = format!("Adding {}", &template.name);
        let spinner = terminal.spinner(&text);
        create(
            &mut manifest,
            &template,
            &config.templates_dir.join(&template.subpath),
        )
        .context(format!("Creating handler \"{}\" failed", self.name))?;
        spinner.finish_with_message(text);

        Ok(())
    }
}

pub fn create(
    manifest: &mut Manifest,
    template: &Template,
    template_path: &Path,
) -> anyhow::Result<()> {
    if manifest.get(&template.name).is_some() {
        anyhow::bail!("Module already exists");
    }
    let to = Path::new(&template.name);
    if to.exists() {
        anyhow::bail!("Module target path is not empty");
    }

    copy_dir(template_path, to)?;

    let handler = Handler::from_template(template);
    manifest.add(handler)?;
    Ok(())
}

pub fn copy_dir(from: &Path, to: &Path) -> anyhow::Result<()> {
    for entry in WalkDir::new(from).into_iter().filter_map(Result::ok) {
        let file_type = entry.file_type();
        let current_path = entry.path().strip_prefix(from)?;
        let target_path = to.join(current_path);

        if file_type.is_dir() {
            fs::create_dir_all(target_path)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target_path)?;
        }
    }
    Ok(())
}
