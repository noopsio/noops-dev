use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};

use serde::Deserialize;

const GITHUB_API_USER: &str = "https://api.github.com/user";
const GITHUB_API_EMAIL: &str = "https://api.github.com/user/emails";

#[derive(Debug, Clone)]
pub struct GithubUser {
    pub id: i32,
    pub email: String,
    pub access_token: String,
}

#[derive(Deserialize)]
struct User {
    id: i32,
}

#[derive(Deserialize)]
struct Email {
    email: String,
    primary: bool,
}

#[cfg_attr(test, faux::create)]
#[derive(Debug, Clone)]
pub struct GithubClient {
    client: Client,
}

#[cfg_attr(test, faux::methods)]
impl GithubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_user(&self, access_token: String) -> anyhow::Result<GithubUser> {
        let headers = self.create_headers(&access_token)?;

        let user_infos = self.get_user_infos(headers.clone()).await?;
        let email = self.get_primary_email(headers).await?;

        Ok(GithubUser {
            id: user_infos.id,
            email: email.email,
            access_token,
        })
    }

    async fn get_user_infos(&self, headers: HeaderMap) -> anyhow::Result<User> {
        let user = self
            .client
            .get(GITHUB_API_USER)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;

        Ok(user)
    }

    async fn get_primary_email(&self, headers: HeaderMap) -> anyhow::Result<Email> {
        let emails: Vec<Email> = self
            .client
            .get(GITHUB_API_EMAIL)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;

        let email = emails.into_iter().find(|email| email.primary).unwrap();
        Ok(email)
    }

    fn create_headers(&self, access_token: &str) -> anyhow::Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "noops-server".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", access_token).parse()?);
        headers.insert(ACCEPT, "application/vnd.github+json".parse()?);
        headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);

        Ok(headers)
    }
}
