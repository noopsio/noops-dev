use crate::{
    adapter::{cargo::CargoAdapter, golang::GolangAdapter},
    client::NoopsClient,
    config::Config,
    modules::{Language, Module},
    terminal::Terminal,
};
use std::path::Path;

pub fn init(term: &Terminal) -> anyhow::Result<()> {
    let project_name = term.text_prompt("Name your Project")?;
    let config = Config::new(&project_name);
    config.save()?;

    term.writeln(&format!("Project {} Initialized", &project_name))?;
    Ok(())
}

pub fn build(term: &Terminal, modules: &[Module]) -> anyhow::Result<()> {
    term.writeln(format!("Building {} modules", modules.len()))?;

    for module in modules.iter() {
        term.writeln(format!("Building module {}", &module.name))?;
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

pub fn deploy(term: &Terminal, modules: &[Module], client: NoopsClient) -> anyhow::Result<()> {
    term.writeln("Deploying project")?;
    if !client.project_exists()? {
        term.writeln("Creating project")?;
        client.create_project()?;
    }
    client.upload_modules(modules)?;
    Ok(())
}

pub fn destroy(term: &Terminal, client: NoopsClient) -> anyhow::Result<()> {
    let response = term.confirm_prompt("Destroying the Project")?;
    if !response {
        term.writeln("Aborting...")?;
        Ok(())
    } else {
        term.writeln("Destroying...")?;
        client.delete_project()?;
        term.writeln("Successfully destroyed project...")?;
        Ok(())
    }
}
