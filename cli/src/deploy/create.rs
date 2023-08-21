use super::{components::BuildedComponent, DeployStep};
use client::function::FunctionClient;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct CreateStep(pub BuildedComponent);

impl DeployStep for CreateStep {
    fn deploy(&self, project: &str, client: &FunctionClient) -> anyhow::Result<()> {
        client.create(project, &self.0.clone().into())?;
        Ok(())
    }
}

impl Display for CreateStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("\t+ {}", &self.0.name);
        let text = style(text.as_str()).green();
        f.write_str(&text.to_string())
    }
}

pub fn create_steps(
    local_modules: &HashSet<BuildedComponent>,
    remote_modules: &HashSet<BuildedComponent>,
) -> Vec<CreateStep> {
    local_modules
        .difference(remote_modules)
        .cloned()
        .map(CreateStep)
        .collect()
}
