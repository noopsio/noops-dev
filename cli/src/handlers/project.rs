use console::style;
use dtos::GetFunctionDTO;

use crate::{
    adapter::{cargo::CargoAdapter, golang::GolangAdapter},
    client::NoopsClient,
    manifest::{Manifest, MANIFEST_FILE_NAME},
    modules::Language,
    terminal::Terminal,
};
use std::path::Path;

use super::diff::ModuleDiff;

pub fn init(term: &Terminal) -> anyhow::Result<()> {
    term.write_heading("Initializing")?;

    if Path::new(MANIFEST_FILE_NAME).exists() {
        term.write_text("Project already initialized")?;
        return Ok(());
    }

    let project_name = term.text_prompt("Project name")?;
    let manifest = Manifest::new(&project_name);
    manifest.save()?;

    term.write_text(format!("{} initialized", &project_name))?;
    Ok(())
}

pub fn build(term: &Terminal, manifest: &Manifest) -> anyhow::Result<()> {
    term.write_heading(format!("Building {}", manifest.project_name))?;

    if manifest.modules.is_empty() {
        term.write_text("No modules to build")?;
        return Ok(());
    }

    for (i, module) in manifest.modules.iter().enumerate() {
        let prefix = format!("[{}/{}]", i + 1, manifest.modules.len());
        let spinner = term.spinner_with_prefix(prefix, &module.name);
        match module.language {
            Language::Rust => {
                let cargo = CargoAdapter::new();
                cargo.build(Path::new(&module.name))?;
            }
            Language::Golang => {
                let go = GolangAdapter::new();
                go.build(Path::new(&module.name))?;
            }
        }
        spinner.finish_with_message(module.name.clone());
    }
    Ok(())
}

pub fn deploy(term: &Terminal, manifest: &Manifest, client: &NoopsClient) -> anyhow::Result<()> {
    term.write_heading(format!("Deploying {}", manifest.project_name))?;

    let mut remote_modules: Vec<GetFunctionDTO> = Default::default();
    let project_exists = client.project_exists()?;

    if project_exists {
        remote_modules = client.project_get()?.functions;
    }

    let diffs = ModuleDiff::new(&manifest.modules, &remote_modules)?;

    if diffs.has_changes() {
        print_changes(&diffs, term)?;
    }
    if diffs.has_not_builds() {
        print_not_build(&diffs, term)?;
    }
    if !diffs.has_changes() && diffs.has_not_builds() {
        return Ok(());
    }
    if !diffs.has_changes() && !diffs.has_not_builds() {
        term.write_text("Project is up to date")?;
        return Ok(());
    }

    if !term.confirm_prompt("Deploying")? {
        term.write_text("Aborting")?;
        return Ok(());
    }

    if !project_exists {
        client.project_create()?;
    }
    deploy_modules(term, &diffs, client)?;

    Ok(())
}

fn deploy_modules(
    term: &Terminal,
    module_diff: &ModuleDiff,
    client: &NoopsClient,
) -> anyhow::Result<()> {
    let mut index = 1;
    let length = module_diff.create.len() + module_diff.update.len() + module_diff.remove.len();
    for (module_name, wasm) in &module_diff.create {
        let prefix = format!("[{}/{}]", index, length);
        let message = format!("Creating {}", &module_name);
        let spinner = term.spinner_with_prefix(prefix, &message);

        client.module_create(module_name, wasm)?;
        spinner.finish_with_message(message);
        index += 1;
    }

    for (module_name, wasm) in &module_diff.update {
        let prefix = format!("[{}/{}]", index, length);
        let message = format!("Updating {}", &module_name);
        let spinner = term.spinner_with_prefix(prefix, &message);

        client.module_update(module_name, wasm)?;
        spinner.finish_with_message(message);
        index += 1;
    }

    for module_name in &module_diff.remove {
        let prefix = format!("[{}/{}]", index, length);
        let message = format!("Removing {}", &module_name);
        let spinner = term.spinner_with_prefix(prefix, &message);

        client.module_delete(module_name)?;
        spinner.finish_with_message(message);
        index += 1;
    }

    Ok(())
}

fn print_changes(diffs: &ModuleDiff, term: &Terminal) -> anyhow::Result<()> {
    term.write_styled_text(style("Changes:").bold())?;

    if !diffs.create.is_empty() {
        for (module_name, _) in &diffs.create {
            let text = format!("\t+ {}", &module_name);
            let text = style(text.as_str()).green();
            term.write_styled_text(text)?;
        }
    }

    if !diffs.update.is_empty() {
        for (module_name, _) in &diffs.update {
            let text = format!("\t~ {}", &module_name);
            let text = style(text.as_str()).yellow();
            term.write_styled_text(text)?;
        }
    }

    if !diffs.remove.is_empty() {
        for module_name in &diffs.remove {
            let text = format!("\t- {}", &module_name);
            let text = style(text.as_str()).red();
            term.write_styled_text(text)?;
        }
    }

    term.write_styled_text(style("---").bold().dim())?;

    Ok(())
}

pub fn print_not_build(diffs: &ModuleDiff, term: &Terminal) -> anyhow::Result<()> {
    term.write_styled_text(style("Not build:").bold())?;

    if !diffs.not_build.is_empty() {
        for module_name in &diffs.not_build {
            let text = format!("\t* {}", &module_name);
            let text = style(text.as_str()).dim();
            term.write_styled_text(text)?;
        }
    }

    term.write_styled_text(style("---").bold().dim())?;
    Ok(())
}

pub fn destroy(term: &Terminal, client: &NoopsClient, project_name: &str) -> anyhow::Result<()> {
    term.write_heading(format!("Destroying {}", project_name))?;

    if !term.confirm_prompt("Destroying")? {
        term.write_text("Aborting")?;
        Ok(())
    } else {
        if !client.project_exists()? {
            term.write_text(format!("{} does not exists", project_name))?;
        } else {
            client.project_delete()?;
            term.write_text(format!("{} destroyed", project_name))?;
        }
        Ok(())
    }
}
