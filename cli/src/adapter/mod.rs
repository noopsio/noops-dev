pub mod cargo;
pub mod git;
pub mod golang;

use anyhow::anyhow;
use std::{path::Path, process::Command};

pub struct BaseAdapter {
    program: String,
}

impl BaseAdapter {
    pub fn new(program: &str) -> Self {
        BaseAdapter {
            program: program.to_string(),
        }
    }

    pub fn build_command(&self, path: &Path, args: &[&str]) -> Command {
        let mut command = Command::new(self.program.clone());
        command.args(args).current_dir(path);
        let command = command;
        command
    }
    pub fn execute(&self, mut command: Command) -> anyhow::Result<()> {
        let output = command.output()?;

        match output.status.code() {
            Some(0) => {
                log::debug!("{} succeeded!", command.get_program().to_string_lossy());
                Ok(())
            }
            Some(code) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let error_message = format!("Command failed with error code {}: {}", code, stderr);
                log::error!("{}", error_message);
                Err(anyhow!(error_message))
            }
            None => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let error_message = format!("Command was terminated by a signal: {}", stderr);
                log::error!("{}", error_message);
                Err(anyhow!(error_message))
            }
        }
    }
}
