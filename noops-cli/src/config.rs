use std::path::Path;

use anyhow;
use serde::{Deserialize, Serialize};

use crate::{modules::Module, print};

pub fn init() -> Config {
    let config_name = print::Color::prompt_text(&print::Color::White, "Name your Project");
    let config = Config::new(&config_name);
    config.to_yaml(None).unwrap();
    config
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub name: String,
    pub modules: Vec<Module>,
}

impl Config {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            modules: std::vec::Vec::new(),
        }
    }

    pub fn from_yaml(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        let file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn add_module(&mut self, module: Module) -> anyhow::Result<()> {
        self.modules.push(module);
        self.to_yaml(None)?;
        Ok(())
    }

    pub fn get_module(&self, index: usize) -> &Module {
        let module = self.modules.get(index).unwrap();
        module
    }

    pub fn delete_module(&mut self, module: usize) -> anyhow::Result<()> {
        self.modules.remove(module);
        self.to_yaml(None)?;
        Ok(())
    }

    pub fn to_yaml(&self, file_name: Option<&str>) -> anyhow::Result<()> {
        let writer;
        let default_file_name = "noops-config.yaml";
        let final_file_name = file_name.unwrap_or(default_file_name);

        writer = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(final_file_name)
            .expect("Couldn't open file");

        serde_yaml::to_writer(writer, &self)?;
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use crate::{config::Config, modules::Module};
    const TEST_CONFIG_PATH: &str = "test/noops-config.yaml";

    #[test]
    fn test_from_yaml() {
        let parsed_config = Config::from_yaml(TEST_CONFIG_PATH).unwrap();

        let mut wanted_config = Config::new("noops-example");
        let example_module = Module::new("my-module", "test/", "my super duper module", "dummy");
        wanted_config.add_module(example_module).unwrap();

        assert_eq!(parsed_config, wanted_config);
    }

    #[test]
    fn test_to_yaml() {
        let config = Config::from_yaml(TEST_CONFIG_PATH).unwrap();
        let saved_config_path = "test/saved.yaml";

        config.to_yaml(Some(saved_config_path)).unwrap();

        let written_config = Config::from_yaml(saved_config_path).unwrap();
        assert_eq!(config, written_config);

        crate::helpers::filesystem::delete_file(saved_config_path)
    }
}
