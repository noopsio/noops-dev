use futures::future::join_all;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    filesystem::{find_wasm, read_wasm},
    modules::Module,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ModuleDTO {
    name: String,
    wasm: Vec<u8>,
    params: Vec<String>,
    project: String,
}

impl From<Module> for ModuleDTO {
    fn from(module: Module) -> Self {
        let path = find_wasm(module.target_dir).unwrap();
        let wasm = read_wasm(path).unwrap();
        ModuleDTO {
            wasm,
            name: module.name,
            ..Default::default()
        }
    }
}

pub struct NoopsClient {
    pub project: String,
    server_url: String,
    client: Client,
}

impl NoopsClient {
    pub fn from_config(config: &Config) -> Self {
        NoopsClient {
            project: config.name.clone(),
            server_url: "http://localhost:3000/api/".to_string(),
            client: Client::new(),
        }
    }
    pub async fn upload_modules(&self, modules: Vec<Module>) {
        let mut uploads = vec![];
        for module in modules {
            uploads.push(self.upload_module(module));
        }
        join_all(uploads).await;
    }

    pub async fn create_project(&self) -> anyhow::Result<()> {
        let project_endpoint = self.server_url.clone() + &self.project;

        log::debug!("Creating project {}", &self.project);

        let response = self.client.post(project_endpoint).send().await?;

        Self::handle_response(response).await?;
        Ok(())
    }

    pub async fn delete_project(&self) -> anyhow::Result<()> {
        let project_endpoint = self.server_url.clone() + &self.project;
        log::debug!("Deleting project {}", &self.project);

        let response = self.client.delete(project_endpoint).send().await?;

        Self::handle_response(response).await?;
        Ok(())
    }

    async fn upload_module(&self, module: Module) -> anyhow::Result<()> {
        let module_endpoint = self.server_url.clone() + &self.project + "/" + &module.name;

        log::debug!("Uploading module {} / {}", &self.project, &module.name);
        log::debug!("Module endpoint {}", module_endpoint);

        let mut payload = ModuleDTO::from(module);
        payload.project = self.project.clone();

        let response = self
            .client
            .post(module_endpoint)
            .json(&payload)
            .send()
            .await?;

        Self::handle_response(response).await?;
        Ok(())
    }

    pub async fn delete_module(&self, module: &Module) -> anyhow::Result<()> {
        let module_endpoint = self.server_url.clone() + &self.project + "/" + &module.name;

        log::debug!("Deleting module {} / {}", &self.project, &module.name);
        log::debug!("Deleting module with endpoint {} ", module_endpoint);

        let response = self.client.delete(module_endpoint).send().await?;

        Self::handle_response(response).await?;
        Ok(())
    }

    async fn handle_response(response: Response) -> anyhow::Result<()> {
        log::debug!("Response status: {}", response.status());

        if !response.status().is_success() {
            let error_message = format!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text().await?
            );
            anyhow::bail!(error_message);
        }
        Ok(())
    }
}