use super::{BuildedComponent, DeployStep};
use client::function::FunctionClient;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct UpdateStep(pub BuildedComponent);

impl DeployStep for UpdateStep {
    fn deploy(&self, project: &str, client: &FunctionClient) -> anyhow::Result<()> {
        client.update(project, &self.0.clone().into())?;
        Ok(())
    }
}

impl Display for UpdateStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("\t~ {}", &self.0.name);
        let text = style(text.as_str()).yellow();
        f.write_str(&text.to_string())
    }
}

pub fn update_steps(
    local_modules: &HashSet<BuildedComponent>,
    remote_modules: &HashSet<BuildedComponent>,
) -> Vec<UpdateStep> {
    let mut local_updates: Vec<BuildedComponent> = local_modules
        .intersection(remote_modules)
        .cloned()
        .collect();
    local_updates.sort();

    let mut remote_updates: Vec<BuildedComponent> = remote_modules
        .intersection(local_modules)
        .cloned()
        .collect();
    remote_updates.sort();

    local_updates
        .iter()
        .zip(remote_updates.iter())
        .filter(|(local_module, remote_module)| local_module.hash != remote_module.hash)
        .map(|(local, _)| UpdateStep(local.clone()))
        .collect()
}
