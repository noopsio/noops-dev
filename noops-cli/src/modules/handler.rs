use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Module {
    pub name: String,
    pub root: std::path::PathBuf,
    pub template: String,
    description: String,
}

impl Module {
    pub fn new(
        name: &str,
        root: impl AsRef<std::path::Path>,
        description: &str,
        template: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            root: root.as_ref().to_path_buf(),
            description: description.to_string(),
            template: template.to_string(),
        }
    }
}

impl From<&ModuleTemplate> for Module {
    fn from(template: &ModuleTemplate) -> Self {
        match template {
            ModuleTemplate {
                index: _,
                ref name,
                ref description,
                ref repository,
            } => Module {
                name: name.to_string(),
                root: std::path::PathBuf::from(&template.name),
                description: description.to_string(),
                template: repository.to_string(),
            },
        }
    }
}

pub struct ModuleTemplate {
    
        index: String,
        pub name: String,
        description: String,
        pub repository: String,
}

// TODO LOAD THESE FROM URL
impl ModuleTemplate {
    pub fn load() -> Vec<ModuleTemplate> {
        vec![ModuleTemplate {
            index: "0".to_string(),
            name: "Rust Hello World".to_string(),
            description: "A hello world function in Rust".to_string(),
            repository: "jfcomputing/templates-rust-hello-world".to_string(),
        },
        ModuleTemplate {
            index: "1".to_string(),
            name: "Golang Hello World".to_string(),
            description: "A hello world function in Golang".to_string(),
            repository: "jfcomputing/templates-go-hello-world".to_string(),
        }]
    }
}

impl Into<Vec<String>> for ModuleTemplate {
    fn into(self) -> Vec<String> {
        match self {
            ModuleTemplate {
                index,
                name,
                description,
                repository,
            } => vec![index, name, description, repository],
        }
    }
}

impl From<&ModuleTemplate> for Vec<String> {
    fn from(template: &ModuleTemplate) -> Vec<String> {
        match template {
            ModuleTemplate {
                index,
                name,
                description,
                repository,
            } => vec![
                index.clone(),
                name.clone(),
                description.clone(),
                repository.clone(),
            ],
        }
    }
}
