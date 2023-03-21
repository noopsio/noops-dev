mod client;
mod config;
mod print;
mod modules;
mod helpers;
mod handlers;

use anyhow::anyhow;
use clap::{command, ArgMatches, Command};
use handlers::{module_delete, project_destroy};
use crate::helpers::Toolchain;
use crate::modules::templates;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = create_arg_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing Project");
            let config = config::init();
            println!("Project Initialized");
            println!("Uploading Project to Server");
            client::NoopsClient::from(&config).create_project().await?;
            Ok(())
        },

        Some(("add", _)) => {
            let config = load_config();
            println!("Creating new module");
            templates::create(config)?;
            Ok(())
        },

        Some(("build", _)) => {
            let config = load_config();
            println!("Building modules");
            helpers::CargoAdapter::build_project(config.modules)?;
            println!("Done");
            Ok(())
        },

        Some(("deploy", _)) => {
            let config = load_config();
            println!("Deploying project");
            client::NoopsClient::from(&config).upload_modules(config.modules).await;
            Ok(())
        },
        Some(("remove", _)) => {
            module_delete().await?;
            Ok(())
        },
        Some(("destroy", _)) => {
            project_destroy().await?;
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

fn load_config() -> config::Config {
    println!("Loading config");
    let config = config::Config::from_yaml("noops-config.yaml").unwrap_or_else(|_| {
        println!("Please init project first with 'noops init' command.");
        std::process::exit(1);
    });
    println!("Successfully loaded config {}", config.name);

    config
}