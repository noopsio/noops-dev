use super::{components::BuildedComponent, DeployStep};
use client::function::FunctionClient;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct DeleteStep(pub BuildedComponent);

impl DeployStep for DeleteStep {
    fn deploy(&self, project: &str, client: &FunctionClient) -> anyhow::Result<()> {
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
    local_modules: &HashSet<BuildedComponent>,
    remote_modules: &HashSet<BuildedComponent>,
) -> Vec<DeleteStep> {
    remote_modules
        .difference(local_modules)
        .cloned()
        .map(DeleteStep)
        .collect()
}
