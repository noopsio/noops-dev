use console::style;
use dtos::GetFunctionDTO;

use crate::{
    adapter::{cargo::CargoAdapter, golang::GolangAdapter},
    client::NoopsClient,
    config::{Config, CONFIG_FILE_NAME},
    modules::Language,
    terminal::Terminal,
};
use std::path::Path;

use super::diff::ModuleDiff;

pub fn init(term: &Terminal) -> anyhow::Result<()> {
    term.write_heading("Initializing")?;

    if Path::new(CONFIG_FILE_NAME).exists() {
        term.write_text("Project already initialized")?;
        return Ok(());
    }

    let project_name = term.text_prompt("Name your Project")?;
    let config = Config::new(&project_name);
    config.save()?;

    term.write_text(format!("{} initialized", &project_name))?;
    Ok(())
}

pub fn build(term: &Terminal, config: &Config) -> anyhow::Result<()> {
    term.write_heading(format!("Building {}", config.project_name))?;

    for (i, module) in config.modules.iter().enumerate() {
        let prefix = format!("[{}/{}]", i + 1, config.modules.len());
        let spinner = term.spinner_with_prefix(prefix, &module.name);
        match module.language {
            Language::Rust => {
                let cargo = CargoAdapter::new();
                cargo.build(Path::new(&module.name))?;
            }
            Language::Golang => {
                let go = GolangAdapter::new();
                go.build(Path::new(&module.name))?;
            }
        }
        spinner.finish_with_message(module.name.clone());
    }
    Ok(())
}

pub fn deploy(term: &Terminal, config: &Config, client: &NoopsClient) -> anyhow::Result<()> {
    term.write_heading(format!("Deploying {}", config.project_name))?;

    let mut remote_modules: Vec<GetFunctionDTO> = Default::default();
    let project_exists = client.project_exists()?;

    if project_exists {
        remote_modules = client.project_get()?;
    }

    let module_diff = ModuleDiff::new(&config.project_name, &config.modules, &remote_modules)?;
    if !module_diff.has_changes() {
        let text = format!(
            "{} {}\nProject is up to date",
            style("Changes:").bold(),
            style("n/a").bold().dim()
        );
        term.write_text(text)?;
        return Ok(());
    }

    print_module_diff(&module_diff, term)?;
    if !term.confirm_prompt("Deploying")? {
        term.write_text("Aborting")?;
        return Ok(());
    }

    if !project_exists {
        client.project_create()?;
    }
    deploy_modules(term, &module_diff, client)?;

    Ok(())
}

fn deploy_modules(
    term: &Terminal,
    module_diff: &ModuleDiff,
    client: &NoopsClient,
) -> anyhow::Result<()> {
    let mut index = 1;
    let length = module_diff.create.len() + module_diff.update.len() + module_diff.remove.len();

    for (module_name, wasm) in &module_diff.create {
        let prefix = format!("[{}/{}]", index, length);
        let message = format!("Creating {}", &module_name);
        let spinner = term.spinner_with_prefix(prefix, &message);

        client.module_create(module_name, wasm)?;
        spinner.finish_with_message(message);
        index += 1;
    }

    for (module_name, wasm) in &module_diff.update {
        let prefix = format!("[{}/{}]", index, length);
        let message = format!("Updating {}", &module_name);
        let spinner = term.spinner_with_prefix(prefix, &message);

        client.module_create(module_name, wasm)?;
        spinner.finish_with_message(message);
        index += 1;
    }

    for module_name in &module_diff.remove {
        let prefix = format!("[{}/{}]", index, length);
        let message = format!("Removing {}", &module_name);
        let spinner = term.spinner_with_prefix(prefix, &message);

        client.module_delete(module_name)?;
        spinner.finish_with_message(message);
        index += 1;
    }

    Ok(())
}

fn print_module_diff(diffs: &ModuleDiff, term: &Terminal) -> anyhow::Result<()> {
    term.write_styled_text(style("Changes:").bold())?;

    if !diffs.create.is_empty() {
        for (module_name, _) in &diffs.create {
            let text = format!("\t+ {}", &module_name);
            let text = style(text.as_str()).green();
            term.write_styled_text(text)?;
        }
    }

    if !diffs.update.is_empty() {
        for (module_name, _) in &diffs.update {
            let text = format!("\t~ {}", &module_name);
            let text = style(text.as_str()).yellow();
            term.write_styled_text(text)?;
        }
    }

    if !diffs.remove.is_empty() {
        for module_name in &diffs.remove {
            let text = format!("\t- {}", &module_name);
            let text = style(text.as_str()).red();
            term.write_styled_text(text)?;
        }
    }

    term.write_styled_text(style("---").bold().dim())?;

    Ok(())
}

pub fn destroy(term: &Terminal, client: &NoopsClient, project_name: &str) -> anyhow::Result<()> {
    term.write_heading(format!("Destroying {}", project_name))?;

    if !term.confirm_prompt("Destroying")? {
        term.write_text("Aborting")?;
        Ok(())
    } else {
        if !client.project_exists()? {
            term.write_text(format!("{} does not exists", project_name))?;
        } else {
            client.project_delete()?;
            term.write_text(format!("{} destroyed", project_name))?;
        }
        Ok(())
    }
}
