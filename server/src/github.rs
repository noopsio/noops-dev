use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};

use serde::Deserialize;

const GITHUB_API_USER: &str = "https://api.github.com/user";
const GITHUB_API_EMAIL: &str = "https://api.github.com/user/emails";

pub struct GithubUser {
    pub id: i32,
    pub email: String,
    pub access_token: String,
}

#[derive(Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct Email {
    email: String,
    primary: bool,
}

pub async fn get_user(access_token: String) -> anyhow::Result<GithubUser> {
    let client = reqwest::Client::new();
    let headers = create_headers(&access_token)?;

    let user_infos = get_user_infos(&client, headers.clone()).await?;
    let email = get_primary_email(&client, headers).await?;

    Ok(GithubUser {
        id: user_infos.id,
        email: email.email,
        access_token,
    })
}

async fn get_user_infos(client: &Client, headers: HeaderMap) -> anyhow::Result<User> {
    let user = client
        .get(GITHUB_API_USER)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(user)
}

async fn get_primary_email(client: &Client, headers: HeaderMap) -> anyhow::Result<Email> {
    let emails: Vec<Email> = client
        .get(GITHUB_API_EMAIL)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    let email = emails.into_iter().find(|email| email.primary).unwrap();
    Ok(email)
}

fn create_headers(access_token: &str) -> anyhow::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "noops-server".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", access_token).parse()?);
    headers.insert(ACCEPT, "application/vnd.github+json".parse()?);
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);

    Ok(headers)
}
