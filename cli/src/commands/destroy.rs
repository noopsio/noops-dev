use std::{fs, path::Path};

use super::Command;
use crate::{config::Config, manifest::Manifest, terminal::Terminal};
use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct DestroyCommand {
    pub name: String,
}

impl Command for DestroyCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let config = Config::default();
        let mut manifest = Manifest::from_yaml(&config.manifest_path)?;

        let text = format!("Removing {}", &self.name);
        let spinner = terminal.spinner(&text);
        destroy(&self.name, &mut manifest)
            .context(format!("Destroying module \"{}\" failed", self.name))?;
        spinner.finish_with_message(text);
        Ok(())
    }
}

pub fn destroy(name: &str, manifest: &mut Manifest) -> anyhow::Result<()> {
    manifest.delete_module_by_name(name)?;
    fs::remove_dir_all(Path::new(name))?;
    Ok(())
}