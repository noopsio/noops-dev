use crate::{
    adapter::{cargo::CargoAdapter, golang::GolangAdapter},
    client::NoopsClient,
    config::Config,
    modules::{Language, Module},
    terminal::Terminal,
};
use std::path::Path;

use super::diff::Diff;

pub fn init(term: &Terminal) -> anyhow::Result<()> {
    let project_name = term.text_prompt("Name your Project")?;
    let config = Config::new(&project_name);
    config.save()?;

    term.writeln(format!("Project {} Initialized", &project_name))?;
    Ok(())
}

pub fn build(term: &Terminal, modules: &[Module]) -> anyhow::Result<()> {
    term.writeln(format!("Building {} modules", modules.len()))?;

    for module in modules.iter() {
        term.writeln(format!("Building {}", &module.name))?;
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
    }
    Ok(())
}

pub fn deploy(term: &Terminal, config: &Config, client: NoopsClient) -> anyhow::Result<()> {
    if !term.confirm_prompt("This will create the project if necessary and upload all modules")? {
        term.writeln("Aborting")?;
        return Ok(());
    }

    if !client.project_exists()? {
        term.writeln("Creating project")?;
        client.project_create()?;
    }

    let remote_modules = client.project_get()?;
    let diffs = Diff::new(&config.project_name, &config.modules, &remote_modules)?;
    print_diff(&diffs, term)?;
    if !term.confirm_prompt("Deploying")? {
        term.writeln("Aborting")?;
        return Ok(());
    }

    for (module_name, wasm) in &diffs.create {
        term.writeln(format!("Creating {}", &module_name))?;
        client.module_create(module_name, wasm)?;
    }

    for (module_name, wasm) in &diffs.update {
        term.writeln(format!("Updating {}", &module_name))?;
        client.module_create(module_name, wasm)?;
    }

    for module_name in &diffs.remove {
        term.writeln(format!("Removing {}", &module_name))?;
        client.module_delete(module_name)?;
    }

    Ok(())
}

fn print_diff(diffs: &Diff, term: &Terminal) -> anyhow::Result<()> {
    if diffs.create.is_empty() {
        term.writeln(format!("Creating {} modules", diffs.create.len()))?;
        for (module_name, _) in &diffs.create {
            term.writeln(format!("\t+ {}", &module_name))?;
        }
    }

    if diffs.update.is_empty() {
        term.writeln(format!("Updating {} modules", &diffs.update.len()))?;
        for (module_name, _) in &diffs.update {
            term.writeln(format!("\t~ {}", &module_name))?;
        }
    }

    if diffs.remove.is_empty() {
        term.writeln(format!("Removing {} modules", &diffs.remove.len()))?;
        for module_name in &diffs.remove {
            term.writeln(format!("\t- {}", &module_name))?;
        }
    }

    Ok(())
}

pub fn destroy(term: &Terminal, client: NoopsClient) -> anyhow::Result<()> {
    if !term.confirm_prompt("Destroying the Project")? {
        term.writeln("Aborting")?;
        Ok(())
    } else {
        client.project_delete()?;
        term.writeln("Successfully destroyed project")?;
        Ok(())
    }
}
