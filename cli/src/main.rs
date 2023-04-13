mod adapter;
mod client;
mod config;
mod filesystem;
mod handlers;
mod modules;
mod templates;
mod terminal;

use adapter::git::GitAdapter;
use clap::{command, Command};
use client::NoopsClient;
use config::Config;
use terminal::Terminal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let terminal = Terminal::new();

    let mut commands = create_commands();
    match commands.clone().get_matches().subcommand() {
        Some(("init", _)) => {
            handlers::project::init(&terminal).await?;
        }

        Some(("build", _)) => {
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            handlers::project::build(&terminal, &config.modules).await?;
        }

        Some(("deploy", _)) => {
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            let client = NoopsClient::from_config(&config);
            handlers::project::deploy(&terminal, &config.modules, client).await?;
        }

        Some(("add", _)) => {
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            let git = GitAdapter::new();
            handlers::modules::add(&terminal, config, &git)?;
        }

        Some(("remove", _)) => {
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            handlers::modules::delete(&terminal, config)?;
        }

        Some(("destroy", _)) => {
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            let client = NoopsClient::from_config(&config);
            handlers::project::destroy(&terminal, client).await?;
        }

        _ => commands.print_help()?,
    }

    Ok(())
}

fn create_commands() -> Command {
    command!()
        .subcommand(Command::new("init").about("Create a new project"))
        .subcommand(Command::new("add").about("Add a new module"))
        .subcommand(Command::new("build").about("Builds all functions in project"))
        .subcommand(Command::new("deploy").about("Deploy the project"))
        .subcommand(Command::new("remove").about("Remove a module"))
        .subcommand(Command::new("destroy").about("Destroy the project"))
}
