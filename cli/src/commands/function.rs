use clap::{Parser, Subcommand};

use crate::{
    adapter::git::GitAdapter,
    handlers,
    manifest::{self, Manifest},
    terminal::Terminal,
};

use super::Command;

#[derive(Subcommand)]
pub enum FunctionCommands {
    /// Create a new function
    Create(FunctionCreateCommand),
    /// Build the function
    Build,
    /// Deploy the function
    Deploy,
    /// Destroy the function
    Destroy(FunctionDestroyCommand),
    /// Show information about the function
    Show,
}

#[derive(Parser, Debug)]
pub struct FunctionCreateCommand;

impl Command for FunctionCreateCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let manifest = Manifest::from_yaml(manifest::MANIFEST_FILE_NAME)?;

        let git = GitAdapter::new();
        handlers::modules::add(&terminal, manifest, &git)?;
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct FunctionDestroyCommand;

impl Command for FunctionDestroyCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let manifest = Manifest::from_yaml(manifest::MANIFEST_FILE_NAME)?;

        handlers::modules::delete(&terminal, manifest)?;
        Ok(())
    }
}
