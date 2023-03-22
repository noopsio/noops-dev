use std::process::Command;
use crate::modules::Module;
use super::Toolchain;

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
        super::execute_command(cargo_build)?;
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

#[cfg(test)]
mod tests {
    use super::{CargoAdapter, Toolchain};
    use crate::modules::Module;

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
}
