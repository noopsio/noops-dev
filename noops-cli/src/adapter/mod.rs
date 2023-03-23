pub mod cargo;
pub mod git;
pub mod golang;

use anyhow::anyhow;
use std::process::Command;

use crate::modules::Module;

pub trait BuildExecutor {
    fn execute_build(&self, target_dir: String) -> anyhow::Result<()>;
}

pub trait LanguageAdapter: BuildExecutor {
    fn new_adapter(modules: Vec<Module>) -> Adapter<Self>
    where
        Self: Sized;
}

pub trait Toolchain {
    fn build_project(&self) -> anyhow::Result<()>;
}

pub struct Adapter<T: BuildExecutor> {
    modules: Vec<Module>,
    build_executor: T,
}

impl<T: BuildExecutor> Adapter<T> {
    pub fn new(modules: Vec<Module>, build_executor: T) -> Self {
        Adapter {
            modules,
            build_executor,
        }
    }
}

impl<T: BuildExecutor> Toolchain for Adapter<T> {
    fn build_project(&self) -> anyhow::Result<()> {
        for module in &self.modules {
            let build_dir = String::from(module.root.to_string_lossy());
            log::debug!("Building dir: {}", build_dir);
            self.build_executor.execute_build(build_dir)?;
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
