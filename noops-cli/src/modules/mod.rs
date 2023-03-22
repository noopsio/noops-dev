pub mod templates;

use self::templates::ModuleTemplate;
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
    pub fn to_vec_string(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.name.clone(),
            self.template.clone(),
            self.description.clone(),
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
                }
            }
        }
    }
}
