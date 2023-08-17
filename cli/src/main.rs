mod adapter;
mod client;
mod commands;
mod config;
mod filesystem;
mod handlers;
mod manifest;
mod modules;
mod templates;
mod terminal;

use clap::Parser;
use commands::Command;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = commands::Cli::parse();

    match &cli {
        commands::Cli::Login(cmd) => cmd.execute()?,
        commands::Cli::Project(project_subcommand) => match project_subcommand {
            commands::project::ProjectCommands::Create(cmd) => cmd.execute()?,
            commands::project::ProjectCommands::Build(cmd) => cmd.execute()?,
            commands::project::ProjectCommands::Deploy(cmd) => cmd.execute()?,
            commands::project::ProjectCommands::Destroy(cmd) => cmd.execute()?,
            commands::project::ProjectCommands::Show => todo!(),
        },
        commands::Cli::Function(function_subcommand) => match function_subcommand {
            commands::function::FunctionCommands::Create(cmd) => cmd.execute()?,
            commands::function::FunctionCommands::Build => unimplemented!(),
            commands::function::FunctionCommands::Deploy => unimplemented!(),
            commands::function::FunctionCommands::Destroy(cmd) => cmd.execute()?,
            commands::function::FunctionCommands::Show => todo!(),
        },
    }
    Ok(())
}
