use std::path::Path;

use crate::adapter::BaseAdapter;

const PROGRAM: &str = "cargo";

pub struct CargoAdapter {
    adapter: BaseAdapter,
}

impl CargoAdapter {
    pub fn new() -> Self {
        CargoAdapter {
            adapter: BaseAdapter::new(PROGRAM),
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
