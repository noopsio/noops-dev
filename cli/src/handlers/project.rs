use dtos::GetFunctionDTO;

use crate::{
    adapter::{cargo::CargoAdapter, golang::GolangAdapter},
    client::NoopsClient,
    config::Config,
    filesystem,
    modules::{Language, Module},
    terminal::Terminal,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

enum DiffOperation {
    Update((String, Vec<u8>)),
    Create((String, Vec<u8>)),
    Delete(String),
}

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
    let diffs = diff(&config.project_name, &config.modules, &remote_modules)?;
    for diff in diffs {
        match diff {
            DiffOperation::Create((module_name, wasm)) => {
                term.writeln(format!("Creating {}", module_name))?;
                client.module_create(&module_name, &wasm)?;
            }
            DiffOperation::Update((module_name, wasm)) => {
                term.writeln(format!("Updating {}", module_name))?;
                client.module_create(&module_name, &wasm)?;
            }
            DiffOperation::Delete(module_name) => {
                term.writeln(format!("Deleting {}", module_name))?;
                client.module_delete(&module_name)?;
            }
        }
    }

    Ok(())
}

fn diff(
    project_name: &str,
    local_modules: &[Module],
    remote_modules: &[GetFunctionDTO],
) -> anyhow::Result<Vec<DiffOperation>> {
    let mut diff: Vec<DiffOperation> = Default::default();
    let mut create_or_change_diff =
        create_or_change_diff(project_name, local_modules, remote_modules)?;
    let mut remove_diff = remove_diff(local_modules, remote_modules)?;

    diff.append(&mut create_or_change_diff);
    diff.append(&mut remove_diff);
    Ok(diff)
}

fn remove_diff(
    local_modules: &[Module],
    remote_modules: &[GetFunctionDTO],
) -> anyhow::Result<Vec<DiffOperation>> {
    let mut diff: Vec<DiffOperation> = Default::default();

    for remote_module in remote_modules {
        let remove = local_modules
            .iter()
            .find(|local_module| remote_module.name == local_module.name);

        if let None = remove {
            diff.push(DiffOperation::Delete(remote_module.name.clone()))
        }
    }

    Ok(diff)
}

fn create_or_change_diff(
    project_name: &str,
    local_modules: &[Module],
    remote_modules: &[GetFunctionDTO],
) -> anyhow::Result<Vec<DiffOperation>> {
    let mut diff: Vec<DiffOperation> = Default::default();
    for local_module in local_modules {
        let create_or_update = remote_modules
            .iter()
            .find(|remote_module| remote_module.name == local_module.name);

        let module_out_path = Path::new(&local_module.name).join("out");
        let module_path = filesystem::find_wasm(module_out_path).unwrap();
        let wasm = filesystem::read_wasm(&module_path)?;

        match create_or_update {
            Some(remote_module) => {
                if remote_module.hash != hash(&project_name, &local_module.name, &wasm) {
                    diff.push(DiffOperation::Update((local_module.name.clone(), wasm)));
                }
            }
            None => diff.push(DiffOperation::Create((local_module.name.clone(), wasm))),
        }
    }
    Ok(diff)
}

fn hash(project_name: &str, module_name: &str, wasm: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    project_name.hash(&mut hasher);
    module_name.hash(&mut hasher);
    wasm.hash(&mut hasher);
    hasher.finish()
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
