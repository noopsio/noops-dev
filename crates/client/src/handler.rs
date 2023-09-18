use common::dtos::{CreateFunctionDTO, GetHandlerDTO};
use reqwest::{blocking::Client as ReqwestClient, header::AUTHORIZATION, StatusCode, Url};

pub struct HandlerClient {
    base_url: Url,
    client: ReqwestClient,
    jwt: String,
}

impl HandlerClient {
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

    pub fn read(&self, project: &str, function: &str) -> anyhow::Result<GetHandlerDTO> {
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

    pub fn read_opt(&self, project: &str, function: &str) -> anyhow::Result<Option<GetHandlerDTO>> {
        let url = self.function_url(project, function)?;

        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
            .send()?;

        if !response.status().is_success() && response.status() != StatusCode::NOT_FOUND {
            anyhow::bail!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?
            );
        }

        if response.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            Ok(Some(response.json()?))
        }
    }

    pub fn exists(&self, project: &str, function: &str) -> anyhow::Result<bool> {
        let url = self.function_url(project, function)?;

        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
            .send()?;

        if !response.status().is_success() && response.status() != StatusCode::NOT_FOUND {
            anyhow::bail!(
                "Request failed with status code {}: {}",
                response.status(),
                response.text()?
            );
        }

        Ok(response.status().is_success() && response.status() != StatusCode::NOT_FOUND)
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
