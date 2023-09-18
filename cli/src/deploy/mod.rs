mod components;
mod create;
mod delete;
mod plan;
mod update;

use self::{components::BuildedComponent, plan::DeployPlan};
use crate::{manifest::Manifest, terminal::Terminal};
use client::{handler::HandlerClient, project::ProjectClient};

trait DeployStep {
    fn deploy(&self, project: &str, client: &HandlerClient) -> anyhow::Result<()>;
}

pub fn deploy_project(
    terminal: &Terminal,
    manifest: Manifest,
    project_client: &ProjectClient,
    handler_client: &HandlerClient,
) -> anyhow::Result<()> {
    terminal.write_heading("Deploying project")?;

    let project = manifest.project_name;
    if !project_client.exists(&project)? {
        project_client.create(&project)?;
    };

    let local_handlers: Vec<BuildedComponent> = manifest
        .handlers
        .iter()
        .filter(|component| component.is_build())
        .cloned()
        .map(|component| BuildedComponent::try_from(component).unwrap())
        .collect();

    let remote_handler: Vec<BuildedComponent> = project_client
        .get(&project)?
        .handlers
        .into_iter()
        .map(BuildedComponent::from)
        .collect();

    let plan = DeployPlan::new(local_handlers, remote_handler);
    prompt_deploy(&plan, terminal, handler_client, &project)?;

    Ok(())
}

pub fn deploy_handler(
    name: &str,
    terminal: &Terminal,
    manifest: Manifest,
    project_client: &ProjectClient,
    handler_client: &HandlerClient,
) -> anyhow::Result<()> {
    terminal.write_heading("Deploying handler")?;

    let project = manifest.project_name.clone();
    if !project_client.exists(&project)? {
        project_client.create(&project)?;
    };

    let local_handler: BuildedComponent = manifest
        .get(name)
        .ok_or(anyhow::anyhow!("Handler not found"))?
        .try_into()?;

    let remote_handler: BuildedComponent = handler_client.read(&project, name)?.into();

    let plan = DeployPlan::new(vec![local_handler], vec![remote_handler]);
    prompt_deploy(&plan, terminal, handler_client, &project)?;
    Ok(())
}

fn prompt_deploy(
    plan: &DeployPlan,
    terminal: &Terminal,
    handler_client: &HandlerClient,
    project: &str,
) -> anyhow::Result<()> {
    if plan.has_steps() {
        terminal.write_text(plan.to_string())?;
        let response = terminal.confirm_prompt("Deploy?")?;
        if response {
            plan.deploy(terminal, project, handler_client)?;
        } else {
            terminal.write_text("Aborting")?;
        }
    } else {
        terminal.write_text("Nothing to deploy")?;
    }
    Ok(())
}
