use crate::{
    client, config,
    helpers::{self, Toolchain},
    print,
};

use super::load_config;

pub async fn project_init() -> anyhow::Result<()> {
    println!("Initializing Project");
    let config = config::init();
    println!("Project Initialized");
    println!("Uploading Project to Server");
    client::NoopsClient::from(&config).create_project().await?;
    Ok(())
}

pub async fn project_build() -> anyhow::Result<()> {
    let config = load_config();
    println!("Building modules");
    helpers::CargoAdapter::build_project(config.modules)?;
    println!("Done");
    Ok(())
}

pub async fn project_deploy() -> anyhow::Result<()> {
    let config = load_config();
    println!("Deploying project");
    client::NoopsClient::from(&config)
        .upload_modules(config.modules)
        .await;
    Ok(())
}

pub async fn project_destroy() -> anyhow::Result<()> {
    let mut answer = print::Color::prompt_text(
        &crate::print::Color::Red,
        "--- \nDestroying the Project! Are you sure? \nYes/ No \n---",
    );
    answer = answer.to_lowercase();

    match &answer[..] {
        "yes" => {
            print::Color::print_colorful(&crate::print::Color::Red, "Destroying...");
            let config = load_config();
            client::NoopsClient::from(&config).delete_project().await?;
            print::Color::print_colorful(
                &crate::print::Color::Green,
                "Successfully destroyed project...",
            );
            Ok(())
        }
        _ => {
            print::Color::print_colorful(&crate::print::Color::Green, "Aborting...");
            Ok(())
        }
    }
}