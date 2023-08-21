use crate::module::{self, BuildFunctionMetadata};
use client::function::FunctionClient;
use common::dtos::CreateFunctionDTO;
use console::style;
use std::{collections::HashSet, fmt::Display};

use super::DeployStep;

#[derive(Debug, Clone, Default)]
pub struct CreateStep(pub BuildFunctionMetadata);

impl DeployStep for CreateStep {
    fn deploy(&self, project: &str, client: &FunctionClient) -> anyhow::Result<()> {
        let function = CreateFunctionDTO {
            name: self.0.name.clone(),
            language: self.0.language,
            wasm: module::wasm(&self.0.name)?,
        };

        client.create(project, &function)?;
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
    local_modules: &HashSet<BuildFunctionMetadata>,
    remote_modules: &HashSet<BuildFunctionMetadata>,
) -> Vec<CreateStep> {
    local_modules
        .difference(remote_modules)
        .cloned()
        .map(CreateStep)
        .collect()
}
