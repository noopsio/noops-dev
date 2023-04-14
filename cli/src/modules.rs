use crate::templates::Template;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Hash, Eq, Copy)]
pub enum Language {
    #[default]
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "golang")]
    Golang,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub language: Language,
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
        }
    }
}
