use log::info;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn read_wasm(path: &Path) -> anyhow::Result<Vec<u8>> {
    info!("File Path {}", path.to_str().unwrap());
    Ok(std::fs::read(path)?)
}

pub fn find_wasm(directory: impl AsRef<Path>) -> Option<PathBuf> {
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.ends_with(".wasm") {
            return Some(entry.into_path());
        }
    }
    None
}

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
