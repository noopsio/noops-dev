use octocrab::models::Author;

pub async fn get_user(access_token: &str) -> anyhow::Result<Author> {
    let octocrab = octocrab::OctocrabBuilder::default()
        .user_access_token(access_token.to_string())
        .build()?;

    Ok(octocrab.current().user().await?)
}
