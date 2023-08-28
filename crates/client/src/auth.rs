use anyhow::Result;
use common::dtos::GetJWTDTO;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AccessToken, AuthUrl, ClientId, DeviceAuthorizationUrl, Scope,
    StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl,
};
use reqwest::{self, StatusCode};
use reqwest::{blocking::Client as ReqwestClient, Url};

const CLIENT_ID: &str = "213ab154663f83fa7e80";
const DEVICE_AUTHORIZATION_URL: &str = "https://github.com/login/device/code";
const AUTHORIZATION_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

pub struct AuthClient {
    url: Url,
    client: ReqwestClient,
}

impl AuthClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            url: Url::parse(base_url).unwrap().join("auth/login").unwrap(),
            client: ReqwestClient::new(),
        }
    }

    pub fn login(&self) -> anyhow::Result<String> {
        let gh_token = get_github_token()?;

        let mut url = self.url.clone();
        url.set_query(Some(&format!("token={}", gh_token.secret())));
        let response: GetJWTDTO = self.client.get(url).send()?.json()?;
        Ok(response.jwt)
    }
}

fn custom_http_client(
    request: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, oauth2::reqwest::Error<reqwest::Error>> {
    let mut response = oauth2::reqwest::http_client(request).unwrap();
    if String::from_utf8(response.body.clone())
        .unwrap()
        .contains("error")
    {
        response.status_code = StatusCode::BAD_REQUEST;
    }
    Ok(response)
}

fn get_github_token() -> anyhow::Result<AccessToken> {
    let device_auth_url =
        DeviceAuthorizationUrl::new(DEVICE_AUTHORIZATION_URL.to_string()).unwrap();
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        None,
        AuthUrl::new(AUTHORIZATION_URL.to_string()).unwrap(),
        Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap()),
    )
    .set_device_authorization_url(device_auth_url);

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .unwrap()
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .request(http_client)
        .map_err(|err| err.to_string())
        .unwrap();

    let uri = details.verification_uri().to_string();
    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        uri,
        details.user_code().secret()
    );

    let token_result = client
        .exchange_device_access_token(&details)
        .request(custom_http_client, std::thread::sleep, None)
        .map_err(|err| err.to_string())
        .unwrap();

    Ok(token_result.access_token().to_owned())
}
