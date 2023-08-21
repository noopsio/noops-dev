use super::DeployStep;
use crate::module::{self, BuildFunctionMetadata};
use client::function::FunctionClient;
use common::dtos::CreateFunctionDTO;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct UpdateStep(pub BuildFunctionMetadata);

impl DeployStep for UpdateStep {
    fn deploy(&self, project: &str, client: &FunctionClient) -> anyhow::Result<()> {
        let function = CreateFunctionDTO {
            name: self.0.name.clone(),
            language: self.0.language,
            wasm: module::wasm(&self.0.name)?,
        };

        client.update(project, &function)?;
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
    local_modules: &HashSet<BuildFunctionMetadata>,
    remote_modules: &HashSet<BuildFunctionMetadata>,
) -> Vec<UpdateStep> {
    let mut local_updates: Vec<BuildFunctionMetadata> = local_modules
        .intersection(remote_modules)
        .cloned()
        .collect();
    local_updates.sort();

    let mut remote_updates: Vec<BuildFunctionMetadata> = remote_modules
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
