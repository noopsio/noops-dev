use super::BuildExecutor;
use std::process::Command;

pub struct CargoExecutor;

impl BuildExecutor for CargoExecutor {
    fn execute_build(&self, target_dir: String) -> anyhow::Result<()> {
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
}

#[cfg(test)]
mod tests {
    use crate::{
        adapter::{Adapter, BuildExecutor, Toolchain},
        modules::Module,
    };

    use super::CargoExecutor;

    const TEST_DIR: &str = "test/";

    #[test]
    fn test_execute_build_cargo() {
        let modules = vec![];
        let cargo_adapter = Adapter::new(modules, CargoExecutor);
        cargo_adapter
            .build_executor
            .execute_build(TEST_DIR.to_string())
            .unwrap()
    }

    #[test]
    fn test_build_project_cargo() {
        let example_module = Module::new(
            "my-module",
            "test/",
            "my super duper module",
            "dummy",
            crate::modules::Language::Rust,
        );
        let modules = vec![example_module];
        let cargo_adapter = Adapter::new(modules, CargoExecutor);

        cargo_adapter.build_project().unwrap();
    }
}
