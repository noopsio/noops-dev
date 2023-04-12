pub mod cargo;
pub mod git;
pub mod golang;

use anyhow::anyhow;
use std::process::Command;

use crate::modules::Module;

pub trait BuildExecutor {
    fn execute_build(&self, target_dir: String) -> anyhow::Result<std::path::PathBuf>;
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
            let build_dir = String::from(module.root.to_string_lossy());
            log::debug!("Building dir: {}", build_dir);
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
