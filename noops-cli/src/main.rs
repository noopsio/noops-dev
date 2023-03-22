mod adapter;
mod client;
mod config;
mod filesystem;
mod handlers;
mod modules;
mod print;

use anyhow::anyhow;
use clap::{command, ArgMatches, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = create_arg_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            handlers::project::project_init().await?;
            Ok(())
        }

        Some(("add", _)) => {
            handlers::modules::module_add()?;
            Ok(())
        }

        Some(("build", _)) => {
            handlers::project::project_build().await?;
            Ok(())
        }

        Some(("deploy", _)) => {
            handlers::project::project_deploy().await?;
            Ok(())
        }
        Some(("remove", _)) => {
            handlers::modules::module_delete().await?;
            Ok(())
        }
        Some(("destroy", _)) => {
            handlers::project::project_destroy().await?;
            Ok(())
        }
        _ => Err(anyhow!("No command provided")),
    }
}

fn create_arg_matches() -> ArgMatches {
    command!()
        .subcommand(Command::new("init").about("Create a new project"))
        .subcommand(Command::new("add").about("Add a new module"))
        .subcommand(Command::new("build").about("Builds all functions in project"))
        .subcommand(Command::new("deploy").about("Deploy the project"))
        .subcommand(Command::new("remove").about("Remove a module"))
        .subcommand(Command::new("destroy").about("Destroy the project"))
        .get_matches()
}
