use std::process::Command;

use super::{Toolchain, execute_command};

struct GolangAdapter;

impl Toolchain for GolangAdapter{
    fn execute_build(target_dir: String) -> anyhow::Result<()> {
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

    fn build_project(modules: Vec<Module>) -> anyhow::Result<()> {
        todo!()
    }
}
