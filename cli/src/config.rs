use crate::modules::Module;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONFIG_FILE_NAME: &str = "./noops.yaml";

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Config {
    #[serde(rename = "project")]
    pub project_name: String,
    pub modules: Vec<Module>,
}

impl Config {
    pub fn new(name: &str) -> Self {
        Self {
            project_name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn from_yaml(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        if !Path::exists(path.as_ref()) {
            anyhow::bail!("Config not found at {}", path.as_ref().to_string_lossy());
        }
        let file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn add_module(&mut self, module: Module) -> anyhow::Result<()> {
        self.modules.push(module);
        self.save()?;
        Ok(())
    }

    pub fn get_module(&self, index: usize) -> Module {
        self.modules.get(index).unwrap().to_owned()
    }

    pub fn delete_module(&mut self, index: usize) -> anyhow::Result<()> {
        self.modules.remove(index);
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
            .truncate(true)
            .open(path)?;

        serde_yaml::to_writer(writer, &self)?;
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use std::fs;

    use super::*;
    use crate::modules::{Language, Module};
    use indoc::formatdoc;
    use lazy_static::lazy_static;
    use tempfile::tempdir;

    const TEST_CONFIG_PATH: &str = "noops.yaml";
    const PROJECT_NAME: &str = "test-project";
    const MODULE_NAME: &str = "test-function";
    const MODULE_DESCRIPTION: &str = "Test module";
    const RUST_MODULE_LANGUAGE: Language = Language::Rust;
    const GO_MODULE_LANGUAGE: Language = Language::Golang;

    lazy_static! {
        static ref CONFIG_CONTENT: String = formatdoc! {"
        project: {PROJECT_NAME}
        modules:
        - name: {MODULE_NAME}
          description: {MODULE_DESCRIPTION}
          language: {RUST_MODULE_LANGUAGE}
        - name: {MODULE_NAME}
          description: {MODULE_DESCRIPTION}
          language: {GO_MODULE_LANGUAGE}
        "};
        static ref CONFIG: Config = Config {
            project_name: PROJECT_NAME.to_string(),
            modules: vec![
                Module {
                    name: MODULE_NAME.to_string(),
                    description: MODULE_DESCRIPTION.to_string(),
                    language: RUST_MODULE_LANGUAGE
                },
                Module {
                    name: MODULE_NAME.to_string(),
                    description: MODULE_DESCRIPTION.to_string(),
                    language: GO_MODULE_LANGUAGE
                },
            ]
        };
    }

    #[test]
    fn from_yaml() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join(TEST_CONFIG_PATH);
        fs::write(&config_path, CONFIG_CONTENT.clone())?;
        let config = Config::from_yaml(config_path)?;
        assert_eq!(config, *CONFIG);
        Ok(())
    }

    #[test]
    fn from_yaml_file_not_found() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join(TEST_CONFIG_PATH);
        let config = Config::from_yaml(config_path);
        assert!(config.is_err());
        Ok(())
    }

    #[test]
    fn to_yaml() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join(TEST_CONFIG_PATH);
        CONFIG.save_to(&config_path)?;
        let written_config = fs::read_to_string(config_path)?;
        assert_eq!(CONFIG_CONTENT.clone(), written_config);
        Ok(())
    }
}
