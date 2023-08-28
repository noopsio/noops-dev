mod components;
mod create;
mod delete;
mod plan;
mod update;

use self::{components::BuildedComponent, plan::DeployPlan};
use crate::{manifest::Manifest, terminal::Terminal};
use client::{function::FunctionClient, project::ProjectClient};

trait DeployStep {
    fn deploy(&self, project: &str, client: &FunctionClient) -> anyhow::Result<()>;
}

pub fn deploy_project(
    terminal: &Terminal,
    manifest: Manifest,
    project_client: &ProjectClient,
    function_client: &FunctionClient,
) -> anyhow::Result<()> {
    terminal.write_heading("Deploying project")?;

    let project = manifest.project_name;
    if !project_client.exists(&project)? {
        project_client.create(&project)?;
    };

    let local_functions: Vec<BuildedComponent> = manifest
        .functions
        .iter()
        .filter(|component| component.is_build())
        .cloned()
        .map(|component| BuildedComponent::try_from(component).unwrap())
        .collect();

    let remote_functions: Vec<BuildedComponent> = project_client
        .get(&project)?
        .functions
        .into_iter()
        .map(BuildedComponent::from)
        .collect();

    let plan = DeployPlan::new(local_functions, remote_functions);
    prompt_deploy(&plan, terminal, function_client, &project)?;

    Ok(())
}

pub fn deploy_function(
    name: &str,
    terminal: &Terminal,
    manifest: Manifest,
    project_client: &ProjectClient,
    function_client: &FunctionClient,
) -> anyhow::Result<()> {
    terminal.write_heading("Deploying function")?;

    let project = manifest.project_name.clone();
    if !project_client.exists(&project)? {
        project_client.create(&project)?;
    };

    let local_function: BuildedComponent = manifest
        .get(name)
        .ok_or(anyhow::anyhow!("Module not found"))?
        .try_into()?;

    let remote_function: BuildedComponent = function_client.read(&project, name)?.into();

    let plan = DeployPlan::new(vec![local_function], vec![remote_function]);
    prompt_deploy(&plan, terminal, function_client, &project)?;
    Ok(())
}

fn prompt_deploy(
    plan: &DeployPlan,
    terminal: &Terminal,
    function_client: &FunctionClient,
    project: &str,
) -> anyhow::Result<()> {
    if plan.has_steps() {
        terminal.write_text(plan.to_string())?;
        let response = terminal.confirm_prompt("Deploy?")?;
        if response {
            plan.deploy(terminal, project, function_client)?;
        } else {
            terminal.write_text("Aborting")?;
        }
    } else {
        terminal.write_text("Nothing to deploy")?;
    }
    Ok(())
}
