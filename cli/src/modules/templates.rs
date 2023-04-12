use super::Language;
use lazy_static::lazy_static;
use std::{fmt::Display, path::PathBuf};

lazy_static! {
    pub static ref TEMPLATES: Vec<ModuleTemplate> = vec![
        ModuleTemplate {
            name: "Rust Hello World".to_string(),
            description: "A hello world function in Rust".to_string(),
            repository: "jfcomputing/templates-rust-hello-world".to_string(),
            language: Language::Rust,
            ..Default::default()
        },
        ModuleTemplate {
            name: "Golang Hello World".to_string(),
            description: "A hello world function in Golang".to_string(),
            repository: "jfcomputing/templates-go-hello-world".to_string(),
            language: Language::Golang,
            ..Default::default()
        },
    ];
}

#[derive(Default, Clone, Debug)]
pub struct ModuleTemplate {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub language: Language,
    pub module_name: Option<String>,
    pub module_root: Option<PathBuf>,
}

impl Display for ModuleTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} - {}", &self.name, &self.description))?;
        Ok(())
    }
}
