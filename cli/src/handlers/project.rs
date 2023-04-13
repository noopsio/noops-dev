use std::path::Path;

use crate::{
    adapter::cargo::CargoAdapter,
    client::NoopsClient,
    config::Config,
    modules::{Language, Module},
    terminal::Terminal,
};

pub async fn init(term: &Terminal) -> anyhow::Result<()> {
    term.writeln("Initializing Project")?;
    let project_name = term.text_prompt("Name your Project")?;
    let config = Config::new(&project_name);
    config.save()?;

    term.writeln("Project Initialized")?;
    term.writeln("Uploading Project to Server")?;
    Ok(())
}

pub async fn build(term: &Terminal, modules: &[Module]) -> anyhow::Result<()> {
    term.writeln("Building modules")?;

    for module in modules.iter() {
        match module.language {
            Language::Rust => {
                let cargo = CargoAdapter::new();
                cargo.build(Path::new(&module.name))?;
            }
            Language::Golang => {}
        }
    }
    Ok(())
}

pub async fn deploy(
    term: &Terminal,
    modules: &[Module],
    client: NoopsClient,
) -> anyhow::Result<()> {
    term.writeln("Deploying project")?;
    client.upload_modules(modules).await;
    Ok(())
}

pub async fn destroy(term: &Terminal, client: NoopsClient) -> anyhow::Result<()> {
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
