use super::execute_command;
use super::BuildExecutor;
use std::path::PathBuf;
use std::process::Command;

pub struct GolangExecutor;

impl BuildExecutor for GolangExecutor {
    fn execute_build(&self, source_dir: String) -> anyhow::Result<std::path::PathBuf> {
        //tinygo build -o target/main.wasm -target wasi main.go
        let build_arg = "build";
        let target_arch_flag = "-target";
        let wasi_arg = "wasi";
        let location_main_go = source_dir.clone() + "src/main.go";
        let output_flag = "-o";
        let output_file = source_dir.clone() + "target/main.wasm";

        let mut tinygo = Command::new("tinygo");
        let tinygo_build = tinygo
            .arg(build_arg)
            .arg(target_arch_flag)
            .arg(wasi_arg)
            .arg(output_flag)
            .arg(output_file)
            .arg(location_main_go);
        execute_command(tinygo_build)?;

        let target_dir = "target";
        let mut binary_location = PathBuf::from(source_dir);
        binary_location.push(target_dir);
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

    use super::GolangExecutor;

    const GOLANG_TEST_DIR: &str = "test/golang/";
    const GOLANG_TARGET_FILE: &str = "test/golang/target/main.wasm";

    #[test]
    fn test_execute_build() {
        let modules = vec![];
        let go_adapter = Adapter::new(modules, GolangExecutor);
        go_adapter
            .build_executor
            .execute_build(GOLANG_TEST_DIR.to_string())
            .unwrap();
        filesystem::delete_file(GOLANG_TARGET_FILE)
    }

    #[test]
    fn test_build_project() {
        let module_name = "my-module";
        let module_description = "my super duper module";
        let template_name = "dummy";
        let module_lang = crate::modules::Language::Golang;
        let module_target_path = PathBuf::default();

        let mut example_module = Module::new(
            module_name,
            GOLANG_TEST_DIR,
            module_description,
            template_name,
            module_lang,
            module_target_path,
        );
        let modules = vec![&mut example_module];
        let mut go_adapter = Adapter::new(modules, GolangExecutor);

        go_adapter.build_project().unwrap();
        filesystem::delete_file(GOLANG_TARGET_FILE)
    }
}
