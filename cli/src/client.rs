use reqwest::blocking::{Client, Response};
use reqwest::{StatusCode, Url};

pub struct NoopsClient {
    pub project: String,
    base_url: Url,
    client: Client,
}

impl NoopsClient {
    pub fn new(base_url: Url, project: &str) -> Self {
        NoopsClient {
            project: project.to_string(),
            base_url,
            client: Client::new(),
        }
    }

    fn get_project_path(&self) -> anyhow::Result<Url> {
        Ok(self.base_url.join(&self.project)?)
    }

    fn get_module_path(&self, module_name: &str) -> anyhow::Result<Url> {
        Ok(self
            .base_url
            .join(&format!("{}/", self.project))?
            .join(module_name)?)
    }

    pub fn project_exists(&self) -> anyhow::Result<bool> {
        let url = self.get_project_path()?;
        let response = reqwest::blocking::get(url)?;
        Ok(response.status().is_success() && response.status() != StatusCode::NOT_FOUND)
    }

    pub fn project_get(&self) -> anyhow::Result<Vec<dtos::GetFunctionDTO>> {
        let url = self.get_project_path()?;
        let response = reqwest::blocking::get(url)?;
        //FIXME
        //Self::handle_response(response)?;
        Ok(response.json()?)
    }

    pub fn project_create(&self) -> anyhow::Result<()> {
        let url = self.get_project_path()?;
        let response = self.client.post(url).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn project_delete(&self) -> anyhow::Result<()> {
        let url = self.get_project_path()?;
        let response = self.client.delete(url).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn module_create(&self, module_name: &str, wasm: &[u8]) -> anyhow::Result<()> {
        let url = self.get_module_path(module_name)?;

        let payload = dtos::CreateFunctionDTO {
            wasm: wasm.to_owned(),
        };

        let response = self.client.post(url).json(&payload).send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn module_delete(&self, module_name: &str) -> anyhow::Result<()> {
        let url = self.get_module_path(module_name)?;
        let response = self.client.delete(url).send()?;
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
