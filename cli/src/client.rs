use crate::{
    filesystem::{find_wasm, read_wasm},
    modules::Module,
};
use futures::future::join_all;
use reqwest::blocking::{Client, Response};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ModuleDTO {
    name: String,
    wasm: Vec<u8>,
    params: Vec<String>,
    project: String,
}

impl From<&Module> for ModuleDTO {
    fn from(module: &Module) -> Self {
        let path = find_wasm(Path::new(&module.name).join("out")).unwrap();
        let wasm = read_wasm(path).unwrap();
        ModuleDTO {
            wasm,
            name: module.name.clone(),
            ..Default::default()
        }
    }
}

pub struct NoopsClient {
    pub project: String,
    base_url: Url,
    client: Client,
}

impl NoopsClient {
    pub fn new(base_url: Url, project: &str) -> Self {
        NoopsClient {
            project: project.to_string(),
            base_url: base_url,
            client: Client::new(),
        }
    }

    pub fn project_exists(&self) -> anyhow::Result<bool> {
        let response = reqwest::blocking::get(self.base_url.as_ref())?;
        Ok(response.status().is_success())
    }

    pub fn upload_modules(&self, modules: &[Module]) -> anyhow::Result<()> {
        for module in modules {
            self.upload_module(module)?;
        }
        Ok(())
    }

    fn upload_module(&self, module: &Module) -> anyhow::Result<()> {
        let module_endpoint = self.base_url.join(&self.project)?.join(&module.name)?;
        let mut payload = ModuleDTO::from(module);
        payload.project = self.project.clone();

        let response = self.client.post(module_endpoint).json(&payload).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn create_project(&self) -> anyhow::Result<()> {
        let project_endpoint = self.base_url.join(&self.project)?;
        let response = self.client.post(project_endpoint).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn delete_project(&self) -> anyhow::Result<()> {
        let project_endpoint = self.base_url.join(&self.project)?;
        let response = self.client.delete(project_endpoint).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn delete_module(&self, module: &Module) -> anyhow::Result<()> {
        let module_endpoint = self.base_url.join(&self.project)?.join(&module.name)?;
        let response = self.client.delete(module_endpoint).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    fn handle_response(response: Response) -> anyhow::Result<()> {
        if !response.status().is_success() {
            let error_message = format!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?
            );
            anyhow::bail!(error_message);
        }
        Ok(())
    }
}
