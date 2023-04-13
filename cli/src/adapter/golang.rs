use super::execute_command;
use super::BuildExecutor;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub struct GolangExecutor;

impl BuildExecutor for GolangExecutor {
    fn execute_build(&self, source_dir: &Path) -> anyhow::Result<std::path::PathBuf> {
        //tinygo build -o target/main.wasm -target wasi main.go
        let build_arg = "build";
        let target_arch_flag = "-target";
        let wasi_arg = "wasi";
        let location_main_go = source_dir.join("src/main.go");
        let output_flag = "-o";
        let output_file = source_dir.join("target/main.wasm");

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
    use super::GolangExecutor;
    use crate::{
        adapter::{Adapter, BuildExecutor, Toolchain},
        modules::Module,
    };
    use std::path::{Path, PathBuf};

    const GOLANG_TEST_DIR: &str = "test/golang/";
    const GOLANG_TARGET_FILE: &str = "test/golang/target/main.wasm";

    #[ignore]
    #[test]
    fn test_execute_build() -> anyhow::Result<()> {
        let modules = vec![];
        let go_adapter = Adapter::new(modules, GolangExecutor);
        go_adapter
            .build_executor
            .execute_build(Path::new(GOLANG_TEST_DIR))?;
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_build_project() -> anyhow::Result<()> {
        let mut example_module = Module {
            name: "my-module".to_string(),
            description: "my super duper module".to_string(),
            language: crate::modules::Language::Rust,
            target_dir: PathBuf::default(),
        };

        let modules = vec![&mut example_module];
        let mut go_adapter = Adapter::new(modules, GolangExecutor);

        go_adapter.build_project().unwrap();
        Ok(())
    }
}
