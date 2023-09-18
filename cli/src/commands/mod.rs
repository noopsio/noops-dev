pub mod build;
pub mod create;
pub mod deploy;
pub mod destroy;
pub mod init;
pub mod login;
pub mod show;
pub mod template;

use self::{
    build::BuildCommand, create::CreateCommand, deploy::DeployCommand, destroy::DestroyCommand,
    init::InitCommand, login::LoginCommand, show::ShowCommand, template::TemplateCommand,
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

    /// Create a handler
    Create(CreateCommand),

    /// Build the project or a handler
    Build(BuildCommand),

    /// Deploy the project or a handler
    Deploy(DeployCommand),

    /// Destroy a handler
    Destroy(DestroyCommand),

    /// Show information about the project or a handler
    Show(ShowCommand),

    /// template
    #[command(subcommand)]
    Template(TemplateCommand),
}
