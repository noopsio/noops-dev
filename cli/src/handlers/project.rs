use crate::{
    adapter::{cargo::CargoAdapter, golang::GolangAdapter},
    client::NoopsClient,
    config::Config,
    filesystem,
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

pub fn deploy(term: &Terminal, config: &Config, client: NoopsClient) -> anyhow::Result<()> {
    if !term.confirm_prompt("This will create the project if necessary and upload all modules")? {
        term.writeln("Aborting")?;
        return Ok(());
    }

    if !client.project_exists()? {
        term.writeln("Creating project")?;
        client.create_project()?;
    }

    term.writeln(format!("Uploading {} modules", config.modules.len()))?;
    for module in config.modules.iter() {
        term.writeln(format!("Uploading module {}", module.name))?;
        let out_path = Path::new(&module.name).join("out");
        let module_path = filesystem::find_wasm(out_path).unwrap();
        let wasm = filesystem::read_wasm(&module_path)?;
        client.create_module(&module.name, &wasm)?;
    }

    Ok(())
}

pub fn destroy(term: &Terminal, client: NoopsClient) -> anyhow::Result<()> {
    if !term.confirm_prompt("Destroying the Project")? {
        term.writeln("Aborting")?;
        Ok(())
    } else {
        client.delete_project()?;
        term.writeln("Successfully destroyed project")?;
        Ok(())
    }
}
