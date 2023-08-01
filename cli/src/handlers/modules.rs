use crate::{
    adapter::git::GitAdapter, filesystem, manifest::Manifest, modules::Module,
    templates::TEMPLATES, terminal::Terminal,
};
use std::{fs, path::Path};
use tempfile::tempdir;

pub fn delete(term: &Terminal, mut manifest: Manifest) -> anyhow::Result<()> {
    term.write_heading("Removing a module")?;

    if manifest.modules.is_empty() {
        term.write_text("No modules to remove")?;
        return Ok(());
    }

    let index = term.select_prompt("Select a modules to delete", &manifest.modules)?;
    let module = manifest.get_module(index);

    let text = format!("Removing {}", &module.name);
    let spinner = term.spinner(&text);

    manifest.delete_module(index)?;
    fs::remove_dir_all(Path::new(&module.name))?;

    spinner.finish_with_message(text);
    Ok(())
}

pub fn add(term: &Terminal, mut manifest: Manifest, git: &GitAdapter) -> anyhow::Result<()> {
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
    manifest.add_module(module)?;

    spinner.finish_with_message(text);
    Ok(())
}
