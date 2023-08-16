use std::{fs, path::Path};
use walkdir::WalkDir;

pub fn copy_dir(from: &Path, to: &Path) -> anyhow::Result<()> {
    for entry in WalkDir::new(from).into_iter().filter_map(Result::ok) {
        let file_type = entry.file_type();
        let current_path = entry.path().strip_prefix(from)?;
        let target_path = to.join(current_path);

        if file_type.is_dir() {
            fs::create_dir_all(target_path)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target_path)?;
        }
    }
    Ok(())
}
