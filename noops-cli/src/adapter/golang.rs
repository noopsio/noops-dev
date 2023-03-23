use super::{execute_command, LanguageAdapter};
use super::{Adapter, BuildExecutor};
use crate::modules::Module;
use std::process::Command;

pub struct GolangExecutor;

impl BuildExecutor for GolangExecutor {
    fn execute_build(&self, target_dir: String) -> anyhow::Result<()> {
        //tinygo build -target wasi main.go
        let mut tinygo = Command::new("tinygo");
        let tinygo_build = tinygo
            .arg("build")
            .arg("-target")
            .arg("wasi")
            .arg(target_dir + "src/main.go");
        execute_command(tinygo_build)?;
        Ok(())
    }
}

impl LanguageAdapter for GolangExecutor {
    fn new_adapter(modules: Vec<Module>) -> Adapter<Self> {
        Adapter::new(modules, Self)
    }
}
