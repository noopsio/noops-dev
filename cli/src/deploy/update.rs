use super::{BuildedComponent, DeployStep};
use client::handler::HandlerClient;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct UpdateStep(pub BuildedComponent);

impl DeployStep for UpdateStep {
    fn deploy(&self, project: &str, client: &HandlerClient) -> anyhow::Result<()> {
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
    local_handlers: &HashSet<BuildedComponent>,
    remote_handlers: &HashSet<BuildedComponent>,
) -> Vec<UpdateStep> {
    let mut local_updates: Vec<BuildedComponent> = local_handlers
        .intersection(remote_handlers)
        .cloned()
        .collect();
    local_updates.sort();

    let mut remote_updates: Vec<BuildedComponent> = remote_handlers
        .intersection(local_handlers)
        .cloned()
        .collect();
    remote_updates.sort();

    local_updates
        .iter()
        .zip(remote_updates.iter())
        .filter(|(local_handler, remote_handler)| local_handler.hash != remote_handler.hash)
        .map(|(local, _)| UpdateStep(local.clone()))
        .collect()
}
