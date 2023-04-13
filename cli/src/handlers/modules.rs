use crate::{
    adapter::git::GitAdapter, config::Config, filesystem, modules::Module, templates::TEMPLATES,
    terminal::Terminal,
};
use std::{fs, path::Path};
use tempfile::tempdir;

pub fn delete(term: &Terminal, mut config: Config) -> anyhow::Result<()> {
    let index = term.select_prompt("Select a module to delete", &config.modules)?;
    let module = config.get_module(index);
    config.delete_module(index)?;
    fs::remove_dir_all(Path::new(&module.name))?;
    Ok(())
}

pub fn add(term: &Terminal, mut config: Config, git: &GitAdapter) -> anyhow::Result<()> {
    let index = term.select_prompt("Select a template", &TEMPLATES)?;
    let mut template = TEMPLATES[index].clone();
    template.name = term.text_prompt("Enter new module name")?;

    let temp_dir = tempdir()?;
    git.get_template(temp_dir.path(), &template.subpath)?;
    filesystem::copy_dir(
        &temp_dir.path().join(&template.subpath),
        Path::new(&template.name),
    )?;

    let module = Module::from_template(template);
    config.add_module(module)?;

    term.writeln("Added module to config!")?;
    Ok(())
}
