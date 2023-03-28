use super::{Adapter, BuildExecutor, LanguageAdapter};
use crate::modules::Module;
use std::process::Command;

pub struct CargoExecutor;

impl BuildExecutor for CargoExecutor {
    fn execute_build(&self, target_dir: String) -> anyhow::Result<()> {
        let build_arg = "build";
        let release_flag = "--release";
        let target_flag = "--target";
        let target_arch = "wasm32-wasi";
        let manifest_flag = "--manifest-path";
        let cargo_toml_path = target_dir + "Cargo.toml";

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
        Ok(())
    }
}

impl LanguageAdapter for CargoExecutor {
    fn new_adapter(modules: Vec<Module>) -> Adapter<Self> {
        Adapter::new(modules, Self)
    }
}

#[cfg(test)]
mod tests {
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
        let example_module = Module::new(
            module_name,
            RUST_TEST_DIR,
            module_description,
            template_name,
            module_lang,
        );
        let modules = vec![example_module];
        let cargo_adapter = Adapter::new(modules, CargoExecutor);

        cargo_adapter.build_project().unwrap();
        filesystem::remove_dir(RUST_TARGET_DIR);
    }
}
