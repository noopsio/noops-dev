mod adapter;
mod client;
mod config;
mod filesystem;
mod handlers;
mod modules;
mod terminal;

use anyhow::anyhow;
use clap::{command, ArgMatches, Command};
use client::NoopsClient;
use config::Config;
use terminal::Terminal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let terminal = Terminal::new();
    let config = Config::from_yaml("noops-config.yaml")?;
    let client = NoopsClient::from_config(&config);

    let matches = create_arg_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            handlers::project::project_init(&terminal).await?;
            Ok(())
        }

        Some(("add", _)) => {
            handlers::modules::module_add(&terminal, config)?;
            Ok(())
        }

        Some(("build", _)) => {
            handlers::project::project_build(&terminal, config).await?;
            Ok(())
        }

        Some(("deploy", _)) => {
            handlers::project::project_deploy(&terminal, config, client).await?;
            Ok(())
        }
        Some(("remove", _)) => {
            handlers::modules::module_delete(&terminal, config, client).await?;
            Ok(())
        }
        Some(("destroy", _)) => {
            handlers::project::project_destroy(&terminal, client).await?;
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
