use super::BuildExecutor;
use std::{path::PathBuf, process::Command};

pub struct CargoExecutor;

impl BuildExecutor for CargoExecutor {
    fn execute_build(&self, source_dir: String) -> anyhow::Result<PathBuf> {
        let build_arg = "build";
        let release_flag = "--release";
        let target_flag = "--target";
        let target_arch = "wasm32-wasi";
        let manifest_flag = "--manifest-path";
        let cargo_toml_path = source_dir.clone() + "Cargo.toml";

        log::debug!("Cargo Toml path: {}", cargo_toml_path);
        let mut cargo = Command::new("cargo");
        let cargo_build = cargo
            .arg(build_arg)
            .arg(release_flag)
            .arg(target_flag)
            .arg(target_arch)
            .arg(manifest_flag)
            .arg(cargo_toml_path);
        super::execute_command(cargo_build)?;

        let target_dir = "target";
        let release_dir = "release";

        let mut binary_location = PathBuf::from(source_dir);
        binary_location.push(target_dir);
        binary_location.push(target_arch);
        binary_location.push(release_dir);

        log::info!("Output Path {}", binary_location.to_string_lossy());

        Ok(binary_location)
    }
}

#[cfg(test)]
mod tests {
    use super::CargoExecutor;
    use crate::{
        adapter::{Adapter, BuildExecutor, Toolchain},
        modules::Module,
    };
    use std::{path::PathBuf, str::FromStr};

    const RUST_TEST_DIR: &str = "test/rust/";

    #[ignore]
    #[test]
    fn test_execute_build() -> anyhow::Result<()> {
        let modules: Vec<&mut Module> = Default::default();
        let cargo_adapter = Adapter::new(modules, CargoExecutor);

        cargo_adapter
            .build_executor
            .execute_build(RUST_TEST_DIR.to_string())?;
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_build_project() -> anyhow::Result<()> {
        let mut example_module = Module {
            name: "my-module".to_string(),
            root: PathBuf::from_str("test/")?,
            description: "my super duper module".to_string(),
            template: "dummy".to_string(),
            language: crate::modules::Language::Rust,
            target_dir: PathBuf::default(),
        };

        let modules = vec![&mut example_module];
        let mut cargo_adapter = Adapter::new(modules, CargoExecutor);

        cargo_adapter.build_project()?;
        Ok(())
    }
}
