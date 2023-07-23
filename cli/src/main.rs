mod adapter;
mod client;
mod commands;
mod config;
mod filesystem;
mod handlers;
mod modules;
mod templates;
mod terminal;

use std::{env, fs, path::Path};

use adapter::git::GitAdapter;
use clap::Parser;
use client::NoopsClient;
use config::Config;
use reqwest::Url;
use terminal::Terminal;

const BASE_URL: &str = "http://localhost:8080/api/";
const CONFIG_NAME: &str = "noops";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let terminal = Terminal::new();
    let xdg_config_home = env::var("XDG_CONFIG_HOME")?;
    let config_path = Path::new(&xdg_config_home).join(CONFIG_NAME);
    fs::create_dir_all(config_path.clone())?;

    let cli = commands::Cli::parse();

    match &cli.commands {
        commands::Commands::Login => {
            let base_url = Url::parse(BASE_URL)?;
            let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
            let client = NoopsClient::new(base_url, config.project_name, None);
            handlers::auth::login(&client, &terminal, &config_path)?;
        }
        commands::Commands::Project(project_subcommand) => match project_subcommand {
            commands::ProjectCommands::Create => handlers::project::init(&terminal)?,
            commands::ProjectCommands::Build => {
                let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
                handlers::project::build(&terminal, &config)?;
            }
            commands::ProjectCommands::Deploy => {
                let base_url = Url::parse(BASE_URL)?;
                let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
                let jwt = handlers::auth::get_jwt(&config_path)?;
                if jwt.is_none() {
                    terminal.write_text("You are not logged in.")?;
                    return Ok(());
                }
                let client = NoopsClient::new(base_url, config.project_name.clone(), jwt);
                handlers::project::deploy(&terminal, &config, &client)?;
            }
            commands::ProjectCommands::Destroy => {
                let base_url = Url::parse(BASE_URL)?;
                let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
                let jwt = handlers::auth::get_jwt(&config_path)?;
                if jwt.is_none() {
                    terminal.write_text("You are not logged in.")?;
                    return Ok(());
                }
                let client = NoopsClient::new(base_url, config.project_name.clone(), jwt);
                handlers::project::destroy(&terminal, &client, &config.project_name)?;
            }
            commands::ProjectCommands::Show => todo!(),
        },
        commands::Commands::Function(function_subcommand) => match function_subcommand {
            commands::FunctionCommands::Create => {
                let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
                let git = GitAdapter::new();
                handlers::modules::add(&terminal, config, &git)?;
            }
            commands::FunctionCommands::Build => todo!(),
            commands::FunctionCommands::Deploy => todo!(),
            commands::FunctionCommands::Destroy => {
                let config = Config::from_yaml(config::CONFIG_FILE_NAME)?;
                handlers::modules::delete(&terminal, config)?;
            }
            commands::FunctionCommands::Show => todo!(),
        },
    }
    Ok(())
}
