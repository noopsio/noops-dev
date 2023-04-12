use std::process::Command;
pub struct GitAdapter;
use std::path::Path;

impl GitAdapter {
    pub fn clone_repository(repository: &str, directory: &Path) -> anyhow::Result<()> {
        println!("Cloning template to {}", directory.to_str().unwrap());
        let mut git = Command::new("git");
        let git_clone = git
            .arg("clone")
            .arg("git@github.com:".to_owned() + repository + ".git")
            .arg(directory.to_str().unwrap());

        super::execute_command(git_clone)?;
        std::fs::remove_dir(directory.join(".git"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_git_clone() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        GitAdapter::clone_repository("JFcomputing/templates-rust-hello-world", temp_dir.path())
            .unwrap();
        assert!(
            std::fs::metadata(temp_dir.path()).unwrap().is_dir(),
            "dir exists"
        );
        Ok(())
    }
}
