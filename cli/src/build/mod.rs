pub mod cargo;
pub mod golang;

use crate::manifest::Handler;
use crate::{manifest::Manifest, terminal::Terminal};
use anyhow::Context;

use common::dtos::Language;
use std::path::Path;

use self::cargo::CargoAdapter;
use self::golang::GolangAdapter;

pub fn build_project(terminal: &Terminal, manifest: &Manifest) -> anyhow::Result<()> {
    terminal.write_heading("Building project")?;

    if manifest.handlers.is_empty() {
        terminal.write_text("No handlers to build")?;
        return Ok(());
    }

    for (i, handler) in manifest.handlers.iter().enumerate() {
        let prefix = format!("[{}/{}]", i + 1, manifest.handlers.len());
        let spinner = terminal.spinner_with_prefix(prefix, &handler.name);

        build(handler).context(format!("Building handler \"{}\" failed", &handler.name))?;
        spinner.finish_with_message(handler.name.clone());
    }
    Ok(())
}

pub fn build_handler(terminal: &Terminal, manifest: &Manifest, name: &str) -> anyhow::Result<()> {
    terminal.write_heading("Building handler")?;

    let text = format!("Building {}", name);
    let spinner = terminal.spinner(&text);
    build_by_name(name, manifest).context(format!("Building handler \"{}\" failed", name))?;
    spinner.finish_with_message(text);
    Ok(())
}

pub fn build_by_name(name: &str, manifest: &Manifest) -> anyhow::Result<()> {
    let handler = manifest
        .get(name)
        .ok_or(anyhow::anyhow!("Handler not found"))?;
    build(&handler)?;
    Ok(())
}

pub fn build(metadata: &Handler) -> anyhow::Result<()> {
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
