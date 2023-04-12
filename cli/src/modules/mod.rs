pub mod templates;

use std::{fmt::Display, path::PathBuf};

use self::templates::ModuleTemplate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Hash, Eq, Copy)]
pub enum Language {
    #[default]
    Rust,
    Golang,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Module {
    pub name: String,
    pub root: PathBuf,
    pub template: String,
    pub description: String,
    pub language: Language,
    pub target_dir: PathBuf,
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        Ok(())
    }
}

impl Module {
    pub fn from_template(template: ModuleTemplate) -> Self {
        Module {
            name: template.name.clone(),
            root: PathBuf::from(template.name),
            description: template.description,
            template: template.repository,
            language: template.language,
            target_dir: Default::default(),
        }
    }
}
