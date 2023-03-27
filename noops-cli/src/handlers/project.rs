use crate::{
    adapter::{cargo::CargoAdapter, Toolchain},
    client, config, print,
};

use super::load_config;

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

pub async fn project_build() -> anyhow::Result<()> {
    let config = load_config();
    println!("Building modules");
    CargoAdapter::build_project(config.modules)?;
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
