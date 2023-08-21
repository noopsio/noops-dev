use common::dtos::{CreateFunctionDTO, GetFunctionDTO};
use reqwest::{blocking::Client as ReqwestClient, header::AUTHORIZATION, Url};

pub struct FunctionClient {
    base_url: Url,
    client: ReqwestClient,
    jwt: String,
}

impl FunctionClient {
    pub fn new(base_url: &str, jwt: String) -> Self {
        Self {
            base_url: Url::parse(base_url).unwrap(),
            client: ReqwestClient::new(),
            jwt,
        }
    }

    pub fn create(&self, project: &str, function: &CreateFunctionDTO) -> anyhow::Result<()> {
        let url = self.function_url(project, &function.name)?;

        let response = self
            .client
            .put(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
            .json(function)
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?,
            );
        }
        Ok(())
    }

    pub fn read(&self, project: &str, function: &str) -> anyhow::Result<GetFunctionDTO> {
        let url = self.function_url(project, function)?;

        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
            .json(function)
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?,
            );
        }
        Ok(response.json()?)
    }

    pub fn update(&self, project: &str, function: &CreateFunctionDTO) -> anyhow::Result<()> {
        let url = self.function_url(project, &function.name)?;

        let response = self
            .client
            .put(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
            .json(function)
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?,
            );
        }
        Ok(())
    }

    pub fn delete(&self, project: &str, function: &str) -> anyhow::Result<()> {
        let url = self.function_url(project, function)?;

        let response = self
            .client
            .delete(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?,
            );
        }
        Ok(())
    }

    fn function_url(&self, project: &str, function: &str) -> anyhow::Result<Url> {
        let url = self
            .base_url
            .join(&(project.to_string() + "/"))?
            .join(function)?;
        Ok(url)
    }
}
