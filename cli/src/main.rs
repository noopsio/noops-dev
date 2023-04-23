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
use reqwest::Url;
use terminal::Terminal;

const BASE_URL: &str = "http://localhost:3000/api/";

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let terminal = Terminal::new();
    let mut commands = create_commands();
    match commands.clone().get_matches().subcommand() {
        Some(("init", _)) => {
            handlers::project::init(&terminal)?;
        }

        Some(("build", _)) => {
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            handlers::project::build(&terminal, &config)?;
        }

        Some(("deploy", _)) => {
            let base_url = Url::parse(BASE_URL)?;
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            let client = NoopsClient::new(base_url, &config.project_name);
            handlers::project::deploy(&terminal, &config, &client)?;
        }

        Some(("destroy", _)) => {
            let base_url = Url::parse(BASE_URL)?;
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            let client = NoopsClient::new(base_url, &config.project_name);
            handlers::project::destroy(&terminal, &client, &config.project_name)?;
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

        _ => commands.print_help()?,
    }

    Ok(())
}

fn create_commands() -> Command {
    command!()
        .subcommand(Command::new("init").about("Create a new project"))
        .subcommand(Command::new("add").about("Add a new module"))
        .subcommand(Command::new("remove").about("Remove a module"))
        .subcommand(Command::new("build").about("Builds all functions in project"))
        .subcommand(Command::new("deploy").about("Deploy the project"))
        .subcommand(Command::new("destroy").about("Destroy the project"))
}
