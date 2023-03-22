pub mod filesystem;

use anyhow::anyhow;
use std::process::Command;

use crate::{modules::Module, helpers::filesystem::remove_dir};

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

pub struct CargoAdapter;

impl Toolchain for CargoAdapter {
    fn execute_build(target_dir: String) -> anyhow::Result<()> {
        let cargo_toml_path = target_dir + "Cargo.toml";
        log::debug!("Cargo Toml path: {}", cargo_toml_path);
        let mut cargo = Command::new("cargo");
        let cargo_build = cargo
            .arg("build")
            .arg("--release")
            .arg("--target")
            .arg("wasm32-wasi")
            .arg("--manifest-path")
            .arg(cargo_toml_path);
        execute_command(cargo_build)?;
        Ok(())
    }

    fn build_project(modules: Vec<Module>) -> anyhow::Result<()> {
        for module in modules {
            let build_dir = String::from(module.root.to_string_lossy());
            log::debug!("Building dir: {}", build_dir);
            CargoAdapter::execute_build(build_dir).unwrap();
        }
        Ok(())
    }
}

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


pub struct GitAdapter;

impl GitAdapter {
    pub fn clone_repository(repository: &str, dir_name: &str) -> anyhow::Result<()> {
        println!("Cloning template to {}", dir_name);
        let mut git = Command::new("git");
        let git_clone = git
            .arg("clone")
            .arg("git@github.com:".to_owned() + repository + ".git")
            .arg(dir_name);

        execute_command(git_clone)?;
        remove_dir(&(dir_name.to_owned() + "/.git"));
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::{modules::Module, helpers::GitAdapter};
    use super::{CargoAdapter, Toolchain};


    #[test]
    fn test_execute_build_cargo() {
        CargoAdapter::execute_build("test/".to_string()).unwrap()
    }

    #[test]
    fn test_build_project_cargo() {
        let example_modules = Module::new("my-module", "test/", "my super duper module", "dummy");
        let modules = vec![example_modules];
        CargoAdapter::build_project(modules).unwrap();
    }

    #[test]
    fn test_git_clone() {
        let cloned_path = "cloned";
        GitAdapter::clone_repository("JFcomputing/templates-rust-hello-world", cloned_path)
            .unwrap();
        assert!(
            std::fs::metadata(cloned_path).unwrap().is_dir(),
            "dir exists"
        );

        crate::helpers::filesystem::remove_dir(cloned_path);
    }
}
