mod adapter;
mod client;
mod config;
mod filesystem;
mod print;
mod templates;
mod modules;

use adapter::Toolchain;
use anyhow::anyhow;
use clap::{command, ArgMatches, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = create_arg_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            config::init()?;
            println!("Project Initialized");

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
            adapter::CargoAdapter::build_project(config.modules)?;
            Ok(())
        },

        Some(("deploy", _)) => {
            let config = load_config();
            println!("Deploying project");
            client::NoopsClient::from(&config).upload_modules(config.modules).await?;
            Ok(())
        },
        _ => Err(anyhow!("No command provided")),
    }
}

fn create_arg_matches() -> ArgMatches {
    command!()
        .subcommand(Command::new("init").about("Create a new project"))
        .subcommand(Command::new("add").about("Add a new module"))
        .subcommand(Command::new("build").about("Builds all functions in project"))
        .subcommand(Command::new("deploy").about("Deploy the project"))
        .get_matches()
}

fn load_config() -> config::Config {
    println!("Loading Config");
    let config = config::Config::from_yaml("noops-config.yaml").unwrap_or_else(|_| {
        println!("Please init project first with 'noops init' command.");
        std::process::exit(1);
    });
    println!("Successfully loaded Config {}", config.name);

    config
}