use std::{fs::File, io::Write, path::Path};

use super::Command;
use crate::config::Config;
use clap::Parser;
use client::auth::AuthClient;

#[derive(Parser, Debug)]
pub struct LoginCommand;

impl Command for LoginCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let config = Config::default();
        let auth_client = AuthClient::new(&config.base_url);
        let jwt = auth_client.login()?;
        set_jwt(&config.jwt_file, &jwt)?;

        Ok(())
    }
}

fn set_jwt(path: &Path, jwt: &str) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(jwt.as_bytes())?;
    Ok(())
}
