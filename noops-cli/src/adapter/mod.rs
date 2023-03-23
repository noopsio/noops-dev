pub mod cargo;
pub mod git;
pub mod golang;

use anyhow::anyhow;
use std::process::Command;

use crate::modules::Module;

pub trait Toolchain {
    fn execute_build(target_dir: String) -> anyhow::Result<()>;
    fn build_project(modules: Vec<Module>) -> anyhow::Result<()>;
}

fn execute_command(command: &mut Command) -> anyhow::Result<()> {
    let command_output = command.output()?;

    match command_output.status.code() {
        Some(0) => {
            log::debug!("{} succeeded!", command.get_program().to_string_lossy());
            Ok(())
        }
        Some(code) => {
            let stderr = String::from_utf8_lossy(&command_output.stderr);
            let error_message = format!("Command failed with error code {}: {}", code, stderr);
            log::debug!("{}", error_message);
            Err(anyhow!(error_message))
        }
        None => {
            let stderr = String::from_utf8_lossy(&command_output.stderr);
            let error_message = format!("Command was terminated by a signal: {}", stderr);
            log::debug!("{}", error_message);
            Err(anyhow!(error_message))
        }
    }
}
