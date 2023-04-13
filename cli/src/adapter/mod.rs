pub mod cargo;
pub mod git;
pub mod golang;

use crate::modules::Module;
use anyhow::anyhow;
use std::{path::Path, process::Command};

pub struct Adapter2 {
    program: String,
}

impl Adapter2 {
    pub fn new(program: &str) -> Self {
        Adapter2 {
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

pub trait BuildExecutor {
    fn execute_build(&self, target_dir: &Path) -> anyhow::Result<std::path::PathBuf>;
}

pub trait Toolchain {
    fn build_project(&mut self) -> anyhow::Result<()>;
}

pub struct Adapter<'a, T: BuildExecutor> {
    modules: Vec<&'a mut Module>,
    build_executor: T,
}

impl<'a, T: BuildExecutor> Adapter<'a, T> {
    pub fn new(modules: Vec<&'a mut Module>, build_executor: T) -> Self {
        Adapter {
            modules,
            build_executor,
        }
    }
}
impl<'a, T: BuildExecutor> Toolchain for Adapter<'a, T> {
    fn build_project(&mut self) -> anyhow::Result<()> {
        for module in &mut self.modules {
            let build_dir = Path::new(&module.name);
            let target_location = self.build_executor.execute_build(build_dir)?;
            module.target_dir = target_location;
        }
        Ok(())
    }
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
            log::error!("{}", error_message);
            Err(anyhow!(error_message))
        }
        None => {
            let stderr = String::from_utf8_lossy(&command_output.stderr);
            let error_message = format!("Command was terminated by a signal: {}", stderr);
            log::error!("{}", error_message);
            Err(anyhow!(error_message))
        }
    }
}
