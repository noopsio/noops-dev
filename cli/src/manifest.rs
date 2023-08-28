use crate::{config::Config, templates::Template};
use common::dtos::Language;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manifest {
    #[serde(rename = "project")]
    pub project_name: String,
    pub functions: Vec<Component>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Component {
    pub name: String,
    pub language: Language,
}

impl Component {
    pub fn from_template(template: &Template) -> Self {
        Self {
            name: template.name.clone(),
            language: template.language,
        }
    }

    pub fn is_build(&self) -> bool {
        self.handler_path().exists()
    }

    pub fn handler_path(&self) -> PathBuf {
        Path::new(&self.name).join("out").join("handler.wasm")
    }
}

impl Manifest {
    pub fn init(name: &str, path: &Path) -> anyhow::Result<()> {
        if path.exists() {
            anyhow::bail!("Project already initialized");
        }

        Self {
            project_name: name.to_string(),
            ..Default::default()
        }
        .save_to(path)?;
        Ok(())
    }

    pub fn from_yaml(path: &Path) -> anyhow::Result<Manifest> {
        if !path.exists() {
            anyhow::bail!("Manifest not found at {}", path.to_string_lossy());
        }
        let file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn add(&mut self, component: Component) -> anyhow::Result<()> {
        self.functions.push(component);
        self.save()?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Component> {
        self.functions
            .iter()
            .cloned()
            .find(|component| component.name == name)
    }

    pub fn delete(&mut self, name: &str) -> anyhow::Result<()> {
        let index = self
            .functions
            .iter()
            .position(|component: &Component| component.name == name)
            .ok_or(anyhow::anyhow!("Module not found"))?;
        self.functions.remove(index);
        self.save()?;

        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        // FIXME Inject config or path
        let config = Config::default();
        self.save_to(&config.manifest_path)?;
        Ok(())
    }

    fn save_to(&self, path: &Path) -> anyhow::Result<()> {
        let writer = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        serde_yaml::to_writer(writer, &self)?;
        Ok(())
    }
}

/*
#[cfg(test)]

mod tests {
    use std::{fs, path::PathBuf};

    use super::*;
    use crate::module::{Language, Module};
    use indoc::formatdoc;
    use lazy_static::lazy_static;
    use tempfile::{tempdir, TempDir};

    const TEST_MANIFEST_PATH: &str = "noops.yaml";
    const PROJECT_NAME: &str = "test-project";
    const MODULE_NAME: &str = "test-function";
    const RUST_MODULE_LANGUAGE: Language = Language::Rust;
    const GO_MODULE_LANGUAGE: Language = Language::Golang;

    lazy_static! {
        static ref MANIFEST_INIT_CONTENT: String = formatdoc! {"
        project: {PROJECT_NAME}
        modules: []
        "};
        static ref MANIFEST_CONTENT: String = formatdoc! {"
        project: {PROJECT_NAME}
        modules:
        - name: {MODULE_NAME}
          language: {RUST_MODULE_LANGUAGE}
        - name: {MODULE_NAME}
          language: {GO_MODULE_LANGUAGE}
        "};
        static ref MANIFEST: Manifest = Manifest {
            project_name: PROJECT_NAME.to_string(),
            modules: vec![
                Module {
                    name: MODULE_NAME.to_string(),
                    language: RUST_MODULE_LANGUAGE,
                    hash: None
                },
                Module {
                    name: MODULE_NAME.to_string(),
                    language: GO_MODULE_LANGUAGE,
                    hash: None,
                },
            ]
        };
    }

    fn setup() -> anyhow::Result<(TempDir, PathBuf)> {
        let temp_dir = tempdir()?;
        let manifest_path = temp_dir.path().join(TEST_MANIFEST_PATH);
        Ok((temp_dir, manifest_path))
    }

    #[test]
    fn init_ok() -> anyhow::Result<()> {
        let (_temp_dir, manifest_path) = setup()?;
        Manifest::init(PROJECT_NAME, &manifest_path)?;
        let manifest = fs::read_to_string(manifest_path)?;
        assert_eq!(MANIFEST_INIT_CONTENT.clone(), manifest);
        Ok(())
    }

    #[test]
    fn init_project_already_initialized() -> anyhow::Result<()> {
        let (_temp_dir, manifest_path) = setup()?;
        Manifest::init(PROJECT_NAME, &manifest_path)?;
        let result = Manifest::init(PROJECT_NAME, &manifest_path);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn from_yaml_ok() -> anyhow::Result<()> {
        let (_temp_dir, manifest_path) = setup()?;

        fs::write(&manifest_path, MANIFEST_CONTENT.clone())?;
        let manifest = Manifest::from_yaml(&manifest_path)?;
        assert_eq!(manifest, *MANIFEST);
        Ok(())
    }

    #[test]
    fn from_yaml_file_not_found() -> anyhow::Result<()> {
        let (_temp_dir, manifest_path) = setup()?;

        let manifest = Manifest::from_yaml(&manifest_path);
        assert!(manifest.is_err());
        Ok(())
    }

    #[test]
    fn to_yaml() -> anyhow::Result<()> {
        let (_temp_dir, manifest_path) = setup()?;

        MANIFEST.save_to(&manifest_path)?;
        let manifest = fs::read_to_string(manifest_path)?;
        assert_eq!(MANIFEST_CONTENT.clone(), manifest);
        Ok(())
    }
}

        */
