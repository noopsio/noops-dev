pub mod function;
pub mod login;
pub mod project;

use self::{function::FunctionCommands, login::LoginCommand, project::ProjectCommands};
use clap::Parser;

pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}

#[derive(Parser)]
#[clap(author, version, about = "noops cli", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Login in the noops cloud
    Login(LoginCommand),

    /// Project subcommands
    #[clap(subcommand)]
    Project(ProjectCommands),

    /// Function subcommands
    #[clap(subcommand)]
    Function(FunctionCommands),
}
