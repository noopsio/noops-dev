pub mod build;
pub mod create;
pub mod deploy;
pub mod destroy;
pub mod init;
pub mod login;
pub mod show;

use self::{
    build::BuildCommand, create::CreateCommand, deploy::DeployCommand, destroy::DestroyCommand,
    init::InitCommand, login::LoginCommand, show::ShowCommand,
};
use clap::Parser;

pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}

#[derive(Parser)]
#[clap(author, version, about = "noops cli", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Initialise a project
    Init(InitCommand),

    /// Login in the noops cloud
    Login(LoginCommand),

    /// Create a function
    Create(CreateCommand),

    /// Build the project or a function
    Build(BuildCommand),

    /// Deploy the project or a function
    Deploy(DeployCommand),

    /// Destroy a function
    Destroy(DestroyCommand),

    /// Show information about the project or a function
    Show(ShowCommand),
}
