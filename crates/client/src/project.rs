use common::dtos;
use reqwest::{blocking::Client as ReqwestClient, header::AUTHORIZATION, StatusCode, Url};

pub struct ProjectClient {
    base_url: Url,
    client: ReqwestClient,
    jwt: String,
}

impl ProjectClient {
    pub fn new(base_url: &str, jwt: String) -> Self {
        Self {
            base_url: Url::parse(base_url).unwrap(),
            client: ReqwestClient::new(),
            jwt,
        }
    }

    pub fn create(&self, name: &str) -> anyhow::Result<()> {
        let url = self.project_url(name)?;

        let response = self
            .client
            .post(url)
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

    pub fn get(&self, name: &str) -> anyhow::Result<dtos::GetProjectDTO> {
        let url = self.project_url(name)?;

        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.jwt))
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

    pub fn _delete(&self, name: &str) -> anyhow::Result<()> {
        let url = self.project_url(name)?;

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

    pub fn exists(&self, name: &str) -> anyhow::Result<bool> {
        let url = self.project_url(name)?;

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

    fn project_url(&self, name: &str) -> anyhow::Result<Url> {
        Ok(self.base_url.join(name)?)
    }
}
