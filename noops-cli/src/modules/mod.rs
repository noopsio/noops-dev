mod templates;

use serde::{Deserialize, Serialize};

use self::templates::ModuleTemplate;

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
