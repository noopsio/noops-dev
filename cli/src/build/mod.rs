pub mod cargo;
pub mod golang;

use crate::manifest::Component;
use crate::{manifest::Manifest, terminal::Terminal};
use anyhow::Context;

use common::dtos::Language;
use std::path::Path;

use self::cargo::CargoAdapter;
use self::golang::GolangAdapter;

pub fn build_project(terminal: &Terminal, manifest: &Manifest) -> anyhow::Result<()> {
    terminal.write_heading("Building project")?;

    if manifest.functions.is_empty() {
        terminal.write_text("No modules to build")?;
        return Ok(());
    }

    for (i, module) in manifest.functions.iter().enumerate() {
        let prefix = format!("[{}/{}]", i + 1, manifest.functions.len());
        let spinner = terminal.spinner_with_prefix(prefix, &module.name);

        build(module).context(format!("Building module \"{}\" failed", &module.name))?;
        spinner.finish_with_message(module.name.clone());
    }
    Ok(())
}

pub fn build_function(terminal: &Terminal, manifest: &Manifest, name: &str) -> anyhow::Result<()> {
    terminal.write_heading("Building function")?;

    let text = format!("Building {}", name);
    let spinner = terminal.spinner(&text);
    build_by_name(name, manifest).context(format!("Building module \"{}\" failed", name))?;
    spinner.finish_with_message(text);
    Ok(())
}

pub fn build_by_name(name: &str, manifest: &Manifest) -> anyhow::Result<()> {
    let module = manifest
        .get(name)
        .ok_or(anyhow::anyhow!("Module not found"))?;
    build(&module)?;
    Ok(())
}

pub fn build(metadata: &Component) -> anyhow::Result<()> {
    match metadata.language {
        Language::Rust => {
            let cargo = CargoAdapter::new();
            cargo.build(Path::new(&metadata.name))?;
        }
        Language::Golang => {
            let go = GolangAdapter::new();
            go.build(Path::new(&metadata.name))?;
        }
    }
    Ok(())
}
