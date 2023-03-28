use super::{execute_command, LanguageAdapter};
use super::{Adapter, BuildExecutor};
use crate::modules::Module;
use std::process::Command;

pub struct GolangExecutor;

impl BuildExecutor for GolangExecutor {
    fn execute_build(&self, target_dir: String) -> anyhow::Result<()> {
        //tinygo build -target wasi main.go
        let build_arg = "build";
        let target_arch_flag = "-target";
        let wasi_arg = "wasi";
        let loc_main_go = "src/main.go";
        let output_flag = "-o";
        let output_dir = target_dir.clone() + "target/main.wasm";

        let mut tinygo = Command::new("tinygo");
        let tinygo_build = tinygo
            .arg(build_arg)
            .arg(target_arch_flag)
            .arg(wasi_arg)
            .arg(output_flag)
            .arg(output_dir)
            .arg(target_dir + loc_main_go);
        execute_command(tinygo_build)?;
        Ok(())
    }
}

impl LanguageAdapter for GolangExecutor {
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
        let example_module = Module::new(
            "my-module",
            GOLANG_TEST_DIR,
            "my super duper module",
            "dummy",
            crate::modules::Language::Golang,
        );
        let modules = vec![example_module];
        let go_adapter = Adapter::new(modules, GolangExecutor);

        go_adapter.build_project().unwrap();
        filesystem::delete_file(GOLANG_TARGET_FILE)
    }
}
