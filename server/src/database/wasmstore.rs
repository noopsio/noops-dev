use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::errors::Error::{self, FunctionAlreadyExists, FunctionNotFound};

#[derive(Debug, Clone)]
pub struct WasmStore {
    prefix: PathBuf,
}

impl WasmStore {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        fs::create_dir_all(path)?;
        Ok(Self {
            prefix: path.to_path_buf(),
        })
    }

    pub fn create_function(&self, function: &str, wasm: &[u8]) -> Result<(), Error> {
        let function = self.prefix.join(format!("{}.wasm", function));
        if function.exists() {
            return Err(FunctionAlreadyExists);
        }
        let mut file = File::create(function).map_err(|err| anyhow::anyhow!(err))?;
        file.write_all(wasm).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn delete_function(&self, function: &str) -> Result<(), Error> {
        let function = self.prefix.join(format!("{}.wasm", function));
        if !function.exists() {
            return Err(FunctionNotFound);
        }
        fs::remove_file(function).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn read_function(&self, function: &str) -> Result<Vec<u8>, Error> {
        let function = self.prefix.join(format!("{}.wasm", function));
        if !function.exists() {
            return Err(FunctionNotFound);
        }
        let wasm = fs::read(function).map_err(|err| anyhow::anyhow!(err))?;
        Ok(wasm)
    }
}
