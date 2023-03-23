use std::process::Command;

use super::{execute_command, BuildExecutor};

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
