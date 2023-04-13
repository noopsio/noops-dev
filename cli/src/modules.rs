use crate::templates::Template;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Hash, Eq, Copy)]
pub enum Language {
    #[default]
    Rust,
    Golang,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Language::Rust => "rust",
            Language::Golang => "golang",
        };
        f.write_str(result)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Module {
    pub name: String,
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
    pub fn from_template(template: Template) -> Self {
        Module {
            name: template.name.clone(),
            description: template.description,
            language: template.language,
            target_dir: Default::default(),
        }
    }
}
