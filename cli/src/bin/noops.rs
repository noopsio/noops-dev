use clap::Parser;
use noops::commands::{self, Command};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = commands::Cli::parse();
    match &cli {
        commands::Cli::Init(cmd) => cmd.execute()?,
        commands::Cli::Login(cmd) => cmd.execute()?,
        commands::Cli::Build(cmd) => cmd.execute()?,
        commands::Cli::Create(cmd) => cmd.execute()?,
        commands::Cli::Deploy(cmd) => cmd.execute()?,
        commands::Cli::Destroy(cmd) => cmd.execute()?,
        commands::Cli::Show(cmd) => cmd.execute()?,
        commands::Cli::Template(cmd) => cmd.execute()?,
    }
    Ok(())
}
