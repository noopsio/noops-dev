use super::BaseAdapter;
use std::path::Path;

const PROGRAM: &str = "git";
const REPOSITORY: &str = "https://github.com/JFComputing/noops-templates.git";

pub struct GitAdapter {
    adapter: BaseAdapter,
}

impl GitAdapter {
    pub fn new() -> Self {
        GitAdapter {
            adapter: BaseAdapter::new(PROGRAM),
        }
    }

    pub fn get_template(&self, working_dir: &Path, path: &Path) -> anyhow::Result<()> {
        self.clone_no_checkout(working_dir, working_dir)?;
        self.sparse_checkout(working_dir, path)?;
        self.checkout(working_dir)?;
        Ok(())
    }

    fn clone_no_checkout(&self, working_dir: &Path, path: &Path) -> anyhow::Result<()> {
        let command = self.adapter.build_command(
            working_dir,
            &[
                "clone",
                "--no-checkout",
                REPOSITORY,
                path.to_string_lossy().as_ref(),
            ],
        );
        self.adapter.execute(command)?;
        Ok(())
    }

    fn sparse_checkout(&self, working_dir: &Path, subpath: &Path) -> anyhow::Result<()> {
        let command = self
            .adapter
            .build_command(working_dir, &["sparse-checkout", "init", "cone"]);
        self.adapter.execute(command)?;

        let command = self.adapter.build_command(
            working_dir,
            &["sparse-checkout", "set", subpath.to_string_lossy().as_ref()],
        );
        self.adapter.execute(command)?;
        Ok(())
    }

    fn checkout(&self, working_dir: &Path) -> anyhow::Result<()> {
        let command = self
            .adapter
            .build_command(working_dir, &["checkout", "main"]);
        self.adapter.execute(command)?;
        Ok(())
    }
}
