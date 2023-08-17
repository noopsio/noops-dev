use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::{client::NoopsClient, terminal::Terminal};
use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AccessToken, AuthUrl, ClientId, DeviceAuthorizationUrl, Scope,
    StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl,
};
use reqwest::{self, StatusCode};

const CLIENT_ID: &str = "213ab154663f83fa7e80";
const DEVICE_AUTHORIZATION_URL: &str = "https://github.com/login/device/code";
const AUTHORIZATION_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const JWT_FILE_NAME: &str = "jwt";

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

fn get_github_token(terminal: &Terminal) -> anyhow::Result<AccessToken> {
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

    // This seams to be a clippy bug
    #[allow(clippy::to_string_in_format_args)]
    #[allow(clippy::unnecessary_to_owned)]
    terminal.write_text(format!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri().to_string(),
        details.user_code().secret()
    ))?;

    let token_result = client
        .exchange_device_access_token(&details)
        .request(custom_http_client, std::thread::sleep, None)
        .map_err(|err| err.to_string())
        .unwrap();

    Ok(token_result.access_token().to_owned())
}

pub fn login(client: &NoopsClient, terminal: &Terminal, path: &Path) -> anyhow::Result<()> {
    let gh_token = get_github_token(terminal)?;
    let jwt = client.login(gh_token.secret())?;
    set_jwt(path, &jwt)?;
    Ok(())
}

pub fn get_jwt(path: &Path) -> anyhow::Result<Option<String>> {
    let path = path.join(JWT_FILE_NAME);
    if !path.exists() {
        return Ok(None);
    }
    let mut jwt = String::default();
    let mut file = File::open(path)?;
    file.read_to_string(&mut jwt)?;
    Ok(Some(jwt))
}

fn set_jwt(path: &Path, jwt: &str) -> anyhow::Result<()> {
    log::debug!("creating jwt dir {}", path.as_os_str().to_string_lossy());
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(path)
        .expect("Could not create dir");
    log::debug!(
        "Writing jwt to file {}",
        path.join(JWT_FILE_NAME).as_os_str().to_string_lossy()
    );

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path.join(JWT_FILE_NAME))
        .expect("Failed to create/open the file");

    file.write_all(jwt.as_bytes())?;
    log::debug!("File sucessfully written!");
    Ok(())
}
