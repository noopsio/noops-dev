mod component;
mod project;

use crate::{
    info::{component::ComponentInformation, project::ProjectInformation},
    manifest::Manifest,
    terminal::Terminal,
};
use client::{function::FunctionClient, project::ProjectClient};
use common::dtos::GetFunctionDTO;

pub fn show_project(
    manifest: &Manifest,
    project_client: &ProjectClient,
    terminal: &Terminal,
) -> anyhow::Result<()> {
    let deployed = project_client.exists(&manifest.project_name)?;
    let mut remote_components: Vec<GetFunctionDTO> = Default::default();

    if deployed {
        remote_components = project_client.get(&manifest.project_name)?.functions;
    }
    let local_components = manifest.functions.clone();

    let component_information: Vec<ComponentInformation> = local_components
        .iter()
        .map(|local_component| {
            let remote_component = remote_components
                .iter()
                .cloned()
                .find(|remote_component| remote_component.name == local_component.name);

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

pub fn show_function(
    name: &str,
    manifest: &Manifest,
    function_client: &FunctionClient,
    terminal: &Terminal,
) -> anyhow::Result<()> {
    let local_component = manifest
        .get(name)
        .ok_or(anyhow::anyhow!("Module not found"))?;
    let remote_component = function_client.read_opt(&manifest.project_name, name)?;

    let component_info = ComponentInformation::new(&local_component, remote_component);

    terminal.write_heading("Showing component")?;
    terminal.write_text(component_info.to_string())?;

    Ok(())
}
