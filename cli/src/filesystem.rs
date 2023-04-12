use anyhow;
use log::info;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn read_wasm(path: PathBuf) -> anyhow::Result<Vec<u8>> {
    info!("File Path {}", path.to_str().unwrap());
    Ok(std::fs::read(path)?)
}

pub fn find_wasm(directory: PathBuf) -> Option<PathBuf> {
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.ends_with(".wasm") {
            return Some(entry.into_path());
        }
    }
    None
}
