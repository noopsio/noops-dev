use clap::Parser;

use crate::{
    client::NoopsClient,
    config::Config,
    handlers,
    manifest::{self, Manifest},
    terminal::Terminal,
};

use super::Command;

#[derive(Parser, Debug)]
pub struct LoginCommand;

impl Command for LoginCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let terminal = Terminal::new();
        let manifest = Manifest::from_yaml(manifest::MANIFEST_FILE_NAME)?;
        let config = Config::default();

        let client = NoopsClient::new(config.base_url, manifest.project_name, None);
        handlers::auth::login(&client, &terminal, &config.jwt_file)?;

        Ok(())
    }
}
