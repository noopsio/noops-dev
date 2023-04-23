use crate::{
    adapter::git::GitAdapter, config::Config, filesystem, modules::Module, templates::TEMPLATES,
    terminal::Terminal,
};
use std::{fs, path::Path};
use tempfile::tempdir;

pub fn delete(term: &Terminal, mut config: Config) -> anyhow::Result<()> {
    term.write_heading("Removing a module")?;

    let index = term.select_prompt("Select a modules to delete", &config.modules)?;
    let module = config.get_module(index);

    let text = format!("Removing {}", &module.name);
    let spinner = term.spinner(&text);

    config.delete_module(index)?;
    fs::remove_dir_all(Path::new(&module.name))?;

    spinner.finish_with_message(text);
    Ok(())
}

pub fn add(term: &Terminal, mut config: Config, git: &GitAdapter) -> anyhow::Result<()> {
    term.write_heading("Adding a module")?;

    let index = term.select_prompt("Select a template", &TEMPLATES)?;
    let mut template = TEMPLATES[index].clone();

    template.name = term.text_prompt("Enter new module name")?;
    let text = format!("Adding {}", &template.name);
    let spinner = term.spinner(&text);

    let temp_dir = tempdir()?;
    git.get_template(temp_dir.path(), &template.subpath)?;
    filesystem::copy_dir(
        &temp_dir.path().join(&template.subpath),
        Path::new(&template.name),
    )?;

    let module = Module::from_template(&template);
    config.add_module(module)?;

    spinner.finish_with_message(text);
    Ok(())
}
