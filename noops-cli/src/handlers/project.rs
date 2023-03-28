use std::collections::HashMap;

use super::load_config;
use crate::{
    adapter::{cargo::CargoExecutor, golang::GolangExecutor, LanguageAdapter, Toolchain},
    client, config,
    modules::{Language, Module},
    print,
};

pub async fn project_init() -> anyhow::Result<()> {
    println!("Initializing Project");
    let config = config::init();
    println!("Project Initialized");
    println!("Uploading Project to Server");
    client::NoopsClient::from_config(&config)
        .create_project()
        .await?;
    Ok(())
}

// projects.rs
pub async fn project_build() -> anyhow::Result<()> {
    let config = load_config();
    println!("Building modules");

    // Group modules based on their language
    let mut grouped_modules: HashMap<Language, Vec<Module>> = HashMap::new();
    for module in config.modules {
        grouped_modules
            .entry(module.language)
            .or_default()
            .push(module);
    }

    for (language, modules) in grouped_modules {
        let adapter: Box<dyn Toolchain> = match language {
            Language::Rust => Box::new(CargoExecutor::new_adapter(modules)),
            Language::Golang => Box::new(GolangExecutor::new_adapter(modules)),
            // Add more languages and their corresponding adapter creators here
        };

        adapter.build_project()?;
    }

    println!("Done");
    Ok(())
}


pub async fn project_deploy() {
    let config = load_config();
    println!("Deploying project");
    client::NoopsClient::from_config(&config)
        .upload_modules(config.modules)
        .await;
}

pub async fn project_destroy() -> anyhow::Result<()> {
    let mut answer = print::Color::prompt_text(
        &print::Color::Red,
        "--- \nDestroying the Project! Are you sure? \nYes/ No \n---",
    );
    answer = answer.to_lowercase();

    match &answer[..] {
        "yes" => {
            print::Color::print_colorful(&print::Color::Red, "Destroying...");
            let config = load_config();
            client::NoopsClient::from_config(&config)
                .delete_project()
                .await?;
            print::Color::print_colorful(&print::Color::Green, "Successfully destroyed project...");
            Ok(())
        }
        _ => {
            print::Color::print_colorful(&print::Color::Green, "Aborting...");
            Ok(())
        }
    }
}
