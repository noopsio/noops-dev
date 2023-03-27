use anyhow::{anyhow, Context};
use log::info;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

pub fn read_binary(file_path: String) -> anyhow::Result<Vec<u8>> {
    info!("File Path {}", file_path);

    let mut file = File::open(file_path)?;
    let mut file_contents = Vec::new();
    let bytes_read = file.read_to_end(&mut file_contents)?;
    info!("bytes read: {}", bytes_read);

    Ok(file_contents)
}

pub fn find_binary(project_root: PathBuf) -> anyhow::Result<String> {
    let mut combined_path = project_root;
    combined_path.push("target/wasm32-wasi/release");
    let path = combined_path.as_path();

    if let Some(entry) = fs::read_dir(path)
        .with_context(|| {
            format!(
                "Failed to read directory '{}'",
                combined_path.as_os_str().to_string_lossy()
            )
        })?
        .filter_map(|entry| entry.ok())
        .find(|entry| {
            let path = entry.path();
            path.is_file() && path.extension().map_or(false, |ext| ext == "wasm")
        })
        .map(|entry| entry.path().to_string_lossy().to_string())
    {
        return Ok(entry);
    }
    Err(anyhow!(
        "No .wasm file found in directory '{}'",
        combined_path.as_os_str().to_string_lossy()
    ))
}

// Test Helpers

#[allow(dead_code)]
pub fn remove_dir(dir: &str) {
    if let Err(e) = std::fs::remove_dir_all(dir) {
        println!("Error removing directory: {}", e);
    } else {
        println!("Directory {} removed successfully", dir);
    }
}

#[allow(dead_code)]
pub fn delete_file(file: &str) {
    match fs::remove_file(file) {
        Ok(_) => println!("File successfully deleted."),
        Err(e) => println!("Error deleting file: {}", e),
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    #[test]
    fn test_find_binary() {
        assert_eq!(
            crate::filesystem::find_binary(PathBuf::from("test")).unwrap(),
            "test/target/wasm32-wasi/release/noops-test-function.wasm"
        );
    }
}
