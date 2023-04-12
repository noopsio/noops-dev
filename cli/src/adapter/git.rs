use std::process::Command;

use crate::filesystem::remove_dir;

pub struct GitAdapter;

impl GitAdapter {
    pub fn clone_repository(repository: &str, dir_name: &str) -> anyhow::Result<()> {
        println!("Cloning template to {}", dir_name);
        let mut git = Command::new("git");
        let git_clone = git
            .arg("clone")
            .arg("git@github.com:".to_owned() + repository + ".git")
            .arg(dir_name);

        super::execute_command(git_clone)?;
        remove_dir(&(dir_name.to_owned() + ".git"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::adapter::git;

    #[test]
    fn test_git_clone() {
        let cloned_path = "cloned";
        git::GitAdapter::clone_repository("JFcomputing/templates-rust-hello-world", cloned_path)
            .unwrap();
        assert!(
            std::fs::metadata(cloned_path).unwrap().is_dir(),
            "dir exists"
        );

        crate::filesystem::remove_dir(cloned_path);
    }
}
