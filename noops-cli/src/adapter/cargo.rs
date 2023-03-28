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
    use std::path::PathBuf;

    use crate::{
        adapter::{Adapter, BuildExecutor, Toolchain},
        filesystem,
        modules::Module,
    };

    use super::CargoExecutor;

    const RUST_TEST_DIR: &str = "test/rust/";
    const RUST_TARGET_DIR: &str = "test/rust/target";

    #[test]
    fn test_execute_build() {
        let modules = vec![];
        let cargo_adapter = Adapter::new(modules, CargoExecutor);
        cargo_adapter
            .build_executor
            .execute_build(RUST_TEST_DIR.to_string())
            .unwrap();
        filesystem::remove_dir(RUST_TARGET_DIR);
    }

    #[test]
    fn test_build_project() {
        let module_name = "my-module";
        let module_description = "my super duper module";
        let template_name = "dummy";
        let module_lang = crate::modules::Language::Rust;
        let module_default_path = PathBuf::default();

        let mut example_module = Module::new(
            module_name,
            RUST_TEST_DIR,
            module_description,
            template_name,
            module_lang,
            module_default_path,
        );
        let modules = vec![&mut example_module];
        let mut cargo_adapter = Adapter::new(modules, CargoExecutor);

        cargo_adapter.build_project().unwrap();
        filesystem::remove_dir(RUST_TARGET_DIR);
    }
}
