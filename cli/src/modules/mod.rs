pub mod templates;

use self::templates::ModuleTemplate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Language {
    Rust,
    Golang,
}

impl Language {
    fn to_string(&self) -> String {
        match self {
            Language::Rust => String::from("Rust"),
            Language::Golang => String::from("Golang"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Module {
    pub name: String,
    pub root: std::path::PathBuf,
    pub template: String,
    description: String,
    pub language: Language,
    pub target_dir: std::path::PathBuf,
}

impl Module {
    pub fn new(
        name: &str,
        root: impl AsRef<std::path::Path>,
        description: &str,
        template: &str,
        language: Language,
        target_dir: impl AsRef<std::path::Path>,
    ) -> Self {
        Self {
            name: name.to_string(),
            root: root.as_ref().to_path_buf(),
            description: description.to_string(),
            template: template.to_string(),
            language,
            target_dir: target_dir.as_ref().to_path_buf(),
        }
    }
    pub fn to_vec_string(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.root.to_string_lossy().to_string(),
            self.template.clone(),
            self.description.clone(),
            self.language.to_string(),
            self.target_dir.to_string_lossy().to_string(),
        ]
    }
}

impl From<ModuleTemplate> for Module {
    fn from(template: ModuleTemplate) -> Self {
        match template {
            ModuleTemplate {
                name: _,
                description,
                repository,
                language,
                module_name,
                module_root,
            } => {
                let module_name = module_name.expect("Module name required");
                Module {
                    name: module_name.clone(),
                    root: module_root
                        .map(|root| std::path::PathBuf::from(root + "/"))
                        .unwrap_or_else(|| std::path::PathBuf::from(module_name + "/")),
                    description: description.to_string(),
                    template: repository.to_string(),
                    language,
                    target_dir: std::path::PathBuf::default(),
                }
            }
        }
    }
}