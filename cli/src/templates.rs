use common::dtos::Language;
use lazy_static::lazy_static;
use std::{fmt::Display, path::PathBuf};

lazy_static! {
    pub static ref TEMPLATES: Vec<Template> = vec![
        Template {
            name: "Rust Hello World".to_string(),
            description: "A hello world function in Rust".to_string(),
            subpath: PathBuf::from("rust/hello-world"),
            language: Language::Rust,
        },
        Template {
            name: "Golang Hello World".to_string(),
            description: "A hello world function in Go".to_string(),
            subpath: PathBuf::from("golang/hello-world"),
            language: Language::Golang,
        }
    ];
}

#[derive(Default, Clone, Debug)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub subpath: PathBuf,
    pub language: Language,
}

impl Template {
    pub fn new(name: String, index: usize) -> Self {
        let mut template = TEMPLATES[index].clone();
        template.name = name;
        template
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} - {}", &self.name, &self.description))?;
        Ok(())
    }
}
