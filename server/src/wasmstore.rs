use crate::errors::Error::{self, FunctionAlreadyExists, HandlerNotFound};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

#[cfg_attr(test, faux::create)]
#[derive(Debug, Clone)]
pub struct WasmStore {
    prefix: PathBuf,
}

#[cfg_attr(test, faux::methods)]
impl WasmStore {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        fs::create_dir_all(path)?;
        Ok(Self {
            prefix: path.to_path_buf(),
        })
    }

    pub fn create(&self, handler_id: &str, wasm: &[u8]) -> Result<(), Error> {
        let path = self.prefix.join(format!("{}.wasm", handler_id));
        if path.exists() {
            return Err(FunctionAlreadyExists);
        }
        self.write(wasm, &path)?;
        Ok(())
    }

    pub fn update(&self, handler_id: &str, wasm: &[u8]) -> Result<(), Error> {
        let path = self.create_path(handler_id);
        self.write(wasm, &path)?;
        Ok(())
    }

    fn write(&self, wasm: &[u8], path: &Path) -> Result<(), Error> {
        let mut file = File::create(path).map_err(|err| anyhow::anyhow!(err))?;
        file.write_all(wasm).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn delete(&self, handler_id: &str) -> Result<(), Error> {
        let path = self.create_path(handler_id);
        if !path.exists() {
            return Err(HandlerNotFound);
        }
        fs::remove_file(path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(())
    }

    pub fn read(&self, handler_id: &str) -> Result<Vec<u8>, Error> {
        let path = self.create_path(handler_id);
        if !path.exists() {
            return Err(HandlerNotFound);
        }
        let wasm = fs::read(path).map_err(|err| anyhow::anyhow!(err))?;
        Ok(wasm)
    }

    fn create_path(&self, handler: &str) -> PathBuf {
        self.prefix.join(format!("{}.wasm", handler))
    }
}
