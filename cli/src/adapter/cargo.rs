use super::Adapter2;
use std::path::{Path, PathBuf};

const PROGRAM: &str = "cargo";

pub struct CargoAdapter {
    adapter: Adapter2,
}

impl CargoAdapter {
    pub fn new() -> Self {
        CargoAdapter {
            adapter: Adapter2 {
                program: PROGRAM.to_string(),
            },
        }
    }

    pub fn build(&self, source_dir: &Path) -> anyhow::Result<PathBuf> {
        let command = self.adapter.build_command(
            source_dir,
            &[
                "build",
                "-Z",
                "unstable-options",
                "--release",
                "--target",
                "wasm32-wasi",
                "--out-dir",
                "./out",
            ],
        );
        self.adapter.execute(command)?;
        let binary = source_dir.join("target").join("release");
        Ok(binary)
    }
}
