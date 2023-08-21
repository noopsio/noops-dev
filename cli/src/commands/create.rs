use super::Command;
use crate::{
    build::BaseAdapter,
    config::Config,
    manifest::Manifest,
    module::FunctionMetadata,
    templates::{Template, TEMPLATES},
    terminal::Terminal,
};
use anyhow::Context;
use clap::Parser;
use std::fs;
use std::path::Path;
use tempfile::tempdir;
use walkdir::WalkDir;

const PROGRAM: &str = "git";
const REPOSITORY: &str = "https://github.com/JFComputing/noops-templates.git";

#[derive(Parser, Debug)]
pub struct CreateCommand {
    pub name: String,
}

impl Command for CreateCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let git = GitAdapter::new();

        let mut manifest = Manifest::from_yaml(&config.manifest_path)?;

        let index = terminal.select_prompt("Select a template", &TEMPLATES)?;
        let template = Template::new(self.name.clone(), index);

        let text = format!("Adding {}", &template.name);
        let spinner = terminal.spinner(&text);
        create(&mut manifest, &git, &template)
            .context(format!("Creating module \"{}\" failed", self.name))?;
        spinner.finish_with_message(text);

        Ok(())
    }
}

pub fn create(
    manifest: &mut Manifest,
    git: &GitAdapter,
    template: &Template,
) -> anyhow::Result<()> {
    if manifest.get_module_by_name(&template.name).is_some() {
        anyhow::bail!("Module already exists");
    }
    let to = Path::new(&template.name);
    if to.exists() {
        anyhow::bail!("Module target path is not empty");
    }

    let temp_dir = tempdir()?;
    git.checkout_template(temp_dir.path(), &template.subpath)?;
    copy_dir(&temp_dir.path().join(&template.subpath), to)?;

    let module = FunctionMetadata::from_template(template);
    manifest.add_module(module)?;
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

#[derive(Clone, Debug, Default)]
pub struct GitAdapter {
    adapter: BaseAdapter,
}

impl GitAdapter {
    pub fn new() -> Self {
        GitAdapter {
            adapter: BaseAdapter::new(PROGRAM),
        }
    }

    pub fn checkout_template(&self, working_dir: &Path, path: &Path) -> anyhow::Result<()> {
        self.clone_no_checkout(working_dir, working_dir)?;
        self.sparse_checkout(working_dir, path)?;
        self.checkout(working_dir)?;
        Ok(())
    }

    fn clone_no_checkout(&self, working_dir: &Path, path: &Path) -> anyhow::Result<()> {
        let command = self.adapter.build_command(
            working_dir,
            &[
                "clone",
                "--no-checkout",
                REPOSITORY,
                path.to_string_lossy().as_ref(),
            ],
        );
        self.adapter.execute(command)?;
        Ok(())
    }

    fn sparse_checkout(&self, working_dir: &Path, subpath: &Path) -> anyhow::Result<()> {
        let command = self
            .adapter
            .build_command(working_dir, &["sparse-checkout", "init", "cone"]);
        self.adapter.execute(command)?;

        let command = self.adapter.build_command(
            working_dir,
            &["sparse-checkout", "set", subpath.to_string_lossy().as_ref()],
        );
        self.adapter.execute(command)?;
        Ok(())
    }

    fn checkout(&self, working_dir: &Path) -> anyhow::Result<()> {
        let command = self
            .adapter
            .build_command(working_dir, &["checkout", "main"]);
        self.adapter.execute(command)?;
        Ok(())
    }
}
