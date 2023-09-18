mod component;
mod project;

use crate::{
    info::{component::ComponentInformation, project::ProjectInformation},
    manifest::Manifest,
    terminal::Terminal,
};
use client::{handler::HandlerClient, project::ProjectClient};
use common::dtos::GetHandlerDTO;

pub fn show_project(
    manifest: &Manifest,
    project_client: &ProjectClient,
    terminal: &Terminal,
) -> anyhow::Result<()> {
    let deployed = project_client.exists(&manifest.project_name)?;
    let mut remote_components: Vec<GetHandlerDTO> = Default::default();

    if deployed {
        remote_components = project_client.get(&manifest.project_name)?.handlers;
    }
    let local_components = manifest.handlers.clone();

    let component_information: Vec<ComponentInformation> = local_components
        .iter()
        .map(|local_component| {
            let remote_component = remote_components
                .iter()
                .find(|remote_component| remote_component.name == local_component.name)
                .cloned();

            ComponentInformation::new(local_component, remote_component)
        })
        .collect();

    let project_info = ProjectInformation::new(
        manifest.project_name.clone(),
        deployed,
        component_information,
    );

    terminal.write_heading("Showing Project")?;
    terminal.write_text(project_info.to_string())?;

    Ok(())
}

pub fn show_handler(
    name: &str,
    manifest: &Manifest,
    handler_client: &HandlerClient,
    terminal: &Terminal,
) -> anyhow::Result<()> {
    let local_component = manifest
        .get(name)
        .ok_or(anyhow::anyhow!("Handler not found"))?;
    let remote_component = handler_client.read_opt(&manifest.project_name, name)?;

    let component_info = ComponentInformation::new(&local_component, remote_component);

    terminal.write_heading("Showing component")?;
    terminal.write_text(component_info.to_string())?;

    Ok(())
}
