pub mod cargo;
pub mod golang;

use crate::manifest::Component;
use crate::{manifest::Manifest, terminal::Terminal};
use anyhow::anyhow;
use anyhow::Context;

use common::dtos::Language;
use std::{path::Path, process::Command};

use self::cargo::CargoAdapter;
use self::golang::GolangAdapter;

#[derive(Clone, Debug, Default)]
pub struct BaseAdapter {
    program: String,
}

impl BaseAdapter {
    pub fn new(program: &str) -> Self {
        BaseAdapter {
            program: program.to_string(),
        }
    }

    pub fn build_command(&self, path: &Path, args: &[&str]) -> Command {
        let mut command = Command::new(self.program.clone());
        command.args(args).current_dir(path);
        command
    }

    pub fn execute(&self, mut command: Command) -> anyhow::Result<()> {
        let output = command.output()?;

        match output.status.code() {
            Some(0) => {
                log::debug!("{} succeeded!", command.get_program().to_string_lossy());
                Ok(())
            }
            Some(code) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let error_message = format!("Command failed with error code {}: {}", code, stderr);
                log::error!("{}", error_message);
                Err(anyhow!(error_message))
            }
            None => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let error_message = format!("Command was terminated by a signal: {}", stderr);
                log::error!("{}", error_message);
                Err(anyhow!(error_message))
            }
        }
    }
}

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
