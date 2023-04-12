use crate::{modules::Module, print};
use anyhow;
use serde::{Deserialize, Serialize};
use std::path::Path;

const CONFIG_FILE_NAME: &str = "noops-config.yaml";

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Config {
    pub name: String,
    pub modules: Vec<Module>,
}

impl Config {
    pub fn init() -> Self {
        let config_name = print::Color::prompt_text(&print::Color::White, "Name your Project");
        let config = Config::new(&config_name);
        config.save().unwrap();
        config
    }

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn from_yaml(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        let file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn add_module(&mut self, module: Module) -> anyhow::Result<()> {
        self.modules.push(module);
        self.save()?;
        Ok(())
    }

    pub fn get_module(&self, index: usize) -> &Module {
        self.modules.get(index).unwrap()
    }

    pub fn delete_module(&mut self, module: usize) -> anyhow::Result<()> {
        self.modules.remove(module);
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        self.save_to(CONFIG_FILE_NAME)?;
        Ok(())
    }

    pub fn save_to(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let writer = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        serde_yaml::to_writer(writer, &self)?;
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::modules::Module;
    use std::path::PathBuf;
    use tempfile::tempdir;

    const TEST_CONFIG_PATH: &str = "test/noops-config.yaml";

    #[test]
    fn test_from_yaml() -> anyhow::Result<()> {
        let mut wanted_config = Config::new("noops-example");

        let example_module = Module::new(
            "my-module",
            "test/",
            "my super duper module",
            "dummy",
            crate::modules::Language::Rust,
            PathBuf::default(),
        );
        wanted_config.add_module(example_module)?;

        let parsed_config = Config::from_yaml(TEST_CONFIG_PATH)?;

        assert_eq!(parsed_config, wanted_config);
        Ok(())
    }

    #[test]
    fn test_to_yaml() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let config = Config::from_yaml(TEST_CONFIG_PATH)?;
        let saved_config_path = temp_dir.path().join("saved.yaml");

        config.save_to(&saved_config_path)?;
        let written_config = Config::from_yaml(&saved_config_path)?;

        assert_eq!(config, written_config);
        Ok(())
    }
}
