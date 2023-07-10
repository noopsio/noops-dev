use dtos::GetJWTDTO;
use reqwest::blocking::{Client, Response};
use reqwest::{header::AUTHORIZATION, StatusCode, Url};

pub struct NoopsClient {
    pub project: String,
    base_url: Url,
    client: Client,
    jwt: Option<String>,
}

impl NoopsClient {
    pub fn new(base_url: Url, project: String, jwt: Option<String>) -> Self {
        NoopsClient {
            project,
            base_url,
            client: Client::new(),
            jwt,
        }
    }

    pub fn login(&self, gh_token: &str) -> anyhow::Result<String> {
        let mut url = self.base_url.join("auth/login")?;
        url.set_query(Some(&format!("token={}", gh_token)));
        let response: GetJWTDTO = self.client.get(url).send()?.json()?;
        Ok(response.jwt)
    }

    pub fn project_get(&self) -> anyhow::Result<Vec<dtos::GetFunctionDTO>> {
        let url = self.get_project_path()?;
        let jwt = self.jwt.clone().unwrap();
        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .send()?;
        //FIXME
        //Self::handle_response(response)?;
        Ok(response.json()?)
    }

    pub fn project_create(&self) -> anyhow::Result<()> {
        let url = self.get_project_path()?;
        let jwt = self.jwt.clone().unwrap();

        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn project_delete(&self) -> anyhow::Result<()> {
        let url = self.get_project_path()?;
        let jwt = self.jwt.clone().unwrap();

        let response = self
            .client
            .delete(url)
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn project_exists(&self) -> anyhow::Result<bool> {
        let url = self.get_project_path()?;
        let jwt = self.jwt.clone().unwrap();
        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .send()?;
        Ok(response.status().is_success() && response.status() != StatusCode::NOT_FOUND)
    }

    fn get_project_path(&self) -> anyhow::Result<Url> {
        Ok(self.base_url.join(&self.project)?)
    }

    pub fn module_create(&self, module_name: &str, wasm: &[u8]) -> anyhow::Result<()> {
        let url = self.get_module_path(module_name)?;
        let payload = dtos::CreateFunctionDTO {
            wasm: wasm.to_owned(),
        };
        let jwt = self.jwt.clone().unwrap();
        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .json(&payload)
            .send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    pub fn module_delete(&self, module_name: &str) -> anyhow::Result<()> {
        let url = self.get_module_path(module_name)?;
        let jwt = self.jwt.clone().unwrap();

        let response = self
            .client
            .delete(url)
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .send()?;
        Self::handle_response(response)?;
        Ok(())
    }

    fn get_module_path(&self, module_name: &str) -> anyhow::Result<Url> {
        Ok(self
            .base_url
            .join(&format!("{}/", self.project))?
            .join(module_name)?)
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
