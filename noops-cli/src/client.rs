use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use futures::future::join_all;


use crate::{
    config::Config,
    helpers::filesystem::{find_binary, read_binary}, modules::Module,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ModuleDTO {
    wasm: Vec<u8>,
}

impl From<Module> for ModuleDTO {
    fn from(module: Module) -> Self {
        let binary_location = find_binary(module.root).unwrap();
        let binary_buffer = read_binary(binary_location).unwrap();
        ModuleDTO {
            wasm: binary_buffer,
        }
    }
}

pub struct NoopsClient {
    project: String,
    server_url: String,
    client: Client,
}

impl From<&Config> for NoopsClient {
    fn from(config: &Config) -> Self {
        NoopsClient {
            project: config.name.clone(),
            server_url: "http://localhost:3000/api/".to_string(),
            client: Client::new(),
        }
    }
}

impl From<&mut Config> for NoopsClient {
    fn from(config: &mut Config) -> Self {
        NoopsClient {
            project: config.name.clone(),
            server_url: "http://localhost:3000/api/".to_string(),
            client: Client::new(),
        }
    }
}

impl NoopsClient {
    pub async fn upload_modules(&self, modules: Vec<Module>) {
        let mut uploads = vec![];
        for module in modules {
            uploads.push(self.upload_module(module));
        }
        join_all(uploads).await;
    }

    pub async fn create_project(&self) -> anyhow::Result<()> {
        let project_endpoint = self.server_url.clone() + &self.project;
        
        log::debug!("Creating Project {}", &self.project);

        let response = self.client
        .post(project_endpoint)
        .send()
        .await?;

        Self::handle_response(response).await?;
        Ok(())
    }
    async fn upload_module(&self, module: Module) -> anyhow::Result<()> {
        let module_endpoint = self.server_url.clone() + &self.project + "/" + &module.name;

        log::debug!("Uploading module {} / {}", &self.project, &module.name);
        log::debug!("Module endpoint {}", module_endpoint);
        
        let payload = ModuleDTO::from(module);

        let response = self.client
        .post(module_endpoint)
        .json(&payload)
        .send()
        .await?;

        Self::handle_response(response).await?;
        Ok(())
    }

    pub async fn delete_module(&self, module: &Module) -> anyhow::Result<()> {
        let module_endpoint = self.server_url.clone() + &self.project + "/" + &module.name;

        log::debug!("Deleting Module {} / {}", &self.project, &module.name);
        
        let response = self.client
        .delete(module_endpoint)
        .send()
        .await?;

        Self::handle_response(response).await?;
        Ok(())
    }

    async fn handle_response(response: Response) -> anyhow::Result<()> {
        if response.status().is_success() {
            println!("Upload succeeded!");
            Ok(())
        } else {
            let error_message = format!(
                "Upload failed with status code {}: {}",
                response.status(),
                response.text().await?
            );
            Err(anyhow::anyhow!(error_message))
        }
    }
}
