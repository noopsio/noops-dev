use super::BaseAdapter;
use std::path::Path;

const PROGRAM: &str = "cargo";

pub struct CargoAdapter {
    adapter: BaseAdapter,
}

impl CargoAdapter {
    pub fn new() -> Self {
        CargoAdapter {
            adapter: BaseAdapter {
                program: PROGRAM.to_string(),
            },
        }
    }

    pub fn build(&self, work_dir: &Path) -> anyhow::Result<()> {
        let command = self.adapter.build_command(
            work_dir,
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
        Ok(())
    }
}
