use super::BaseAdapter;
use std::{fs, path::Path};

const PROGRAM: &str = "tinygo";

pub struct GolangAdapter {
    adapter: BaseAdapter,
}

impl GolangAdapter {
    pub fn new() -> Self {
        GolangAdapter {
            adapter: BaseAdapter {
                program: PROGRAM.to_string(),
            },
        }
    }

    pub fn build(&self, work_dir: &Path) -> anyhow::Result<()> {
        let out_path = work_dir.join("out");
        if !out_path.exists() {
            fs::create_dir(&out_path)?;
        }
        let command = self.adapter.build_command(
            work_dir,
            &["build", "-target", "wasi", "-o", "./out/handler.wasm"],
        );
        self.adapter.execute(command)?;
        Ok(())
    }
}