use super::{components::BuildedComponent, DeployStep};
use client::handler::HandlerClient;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct DeleteStep(pub BuildedComponent);

impl DeployStep for DeleteStep {
    fn deploy(&self, project: &str, client: &HandlerClient) -> anyhow::Result<()> {
        client.delete(project, &self.0.name)?;
        Ok(())
    }
}

impl Display for DeleteStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("\t- {}", &self.0.name);
        let text = style(text.as_str()).red();
        f.write_str(&text.to_string())
    }
}

pub fn delete_steps(
    local_handlers: &HashSet<BuildedComponent>,
    remote_handlers: &HashSet<BuildedComponent>,
) -> Vec<DeleteStep> {
    remote_handlers
        .difference(local_handlers)
        .cloned()
        .map(DeleteStep)
        .collect()
}
