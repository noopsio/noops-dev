use std::collections::HashMap;

use crate::{
    adapter::{cargo::CargoExecutor, golang::GolangExecutor, Adapter, Toolchain},
    client::{self, NoopsClient},
    config::Config,
    modules::{Language, Module},
    terminal::Terminal,
};

pub async fn project_init(term: &Terminal) -> anyhow::Result<()> {
    term.writeln("Initializing Project")?;
    let project_name = term.text_prompt("Name your Project")?;
    let config = Config::new(&project_name);
    config.save()?;

    term.writeln("Project Initialized")?;
    term.writeln("Uploading Project to Server")?;

    client::NoopsClient::from_config(&config)
        .create_project()
        .await?;
    Ok(())
}

pub async fn project_build(term: &Terminal, mut config: Config) -> anyhow::Result<()> {
    term.writeln("Building modules")?;

    // Group modules based on their language
    let mut grouped_modules: HashMap<Language, Vec<&mut Module>> = HashMap::new();
    for module in config.modules.iter_mut() {
        grouped_modules
            .entry(module.language)
            .or_default()
            .push(module);
    }

    for (language, modules) in grouped_modules {
        let mut adapter: Box<dyn Toolchain> = match language {
            Language::Rust => {
                let executor = CargoExecutor;
                let adapter = Adapter::new(modules, executor);
                Box::new(adapter)
            }
            Language::Golang => {
                let executor = GolangExecutor;
                let adapter = Adapter::new(modules, executor);
                Box::new(adapter)
            } // Add more languages and their corresponding adapter creators here
        };

        adapter.build_project()?;
    }

    config.save()?;
    term.writeln("Done")?;
    Ok(())
}

pub async fn project_deploy(
    term: &Terminal,
    config: Config,
    client: NoopsClient,
) -> anyhow::Result<()> {
    term.writeln("Deploying project")?;
    client.upload_modules(config.modules).await;
    Ok(())
}

pub async fn project_destroy(term: &Terminal, client: NoopsClient) -> anyhow::Result<()> {
    let response = term.confirm_prompt("Destroying the Project")?;
    if !response {
        term.writeln("Aborting...")?;
        Ok(())
    } else {
        term.writeln("Destroying...")?;
        client.delete_project().await?;
        term.writeln("Successfully destroyed project...")?;
        Ok(())
    }
}
