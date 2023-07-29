use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "noops cli", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login in the noops cloud
    Login,

    /// Project subcommands
    #[command(subcommand)]
    Project(ProjectCommands),

    /// Function subcommands
    #[command(subcommand)]
    Function(FunctionCommands),
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Create a new project
    Create,
    /// Build the project
    Build,
    /// Deploy the function
    Deploy,
    /// Destroy the project
    Destroy,
    /// Show information about the project
    Show,
}

#[derive(Subcommand)]
pub enum FunctionCommands {
    /// Create a new function
    Create,
    /// Build the function
    Build,
    /// Deploy the function
    Deploy,
    /// Destroy the function
    Destroy,
    /// Show information about the function
    Show,
}
