use crate::adapter::BaseAdapter;
use anyhow::Result;
use common::dtos::Language;
use serde::Deserialize;
use std::{fmt::Display, path::PathBuf};
use std::{fs, path::Path};

const PROGRAM: &str = "git";
const REPOSITORY: &str = "https://github.com/noopsio/noops-templates.git";

#[derive(Default, Clone, Debug, Deserialize)]
pub struct TemplateManifest {
    pub templates: Vec<Template>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub subpath: PathBuf,
    pub language: Language,
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} - {}", &self.name, &self.description))?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct TemplateManager {
    adapter: BaseAdapter,
}

impl TemplateManager {
    pub fn new() -> Self {
        TemplateManager {
            adapter: BaseAdapter::new(PROGRAM),
        }
    }

    pub fn update(&self, template_dir: &Path) -> Result<()> {
        let command = if template_dir.exists() {
            self.adapter.build_command(template_dir, &["pull"])
        } else {
            fs::create_dir(template_dir)?;
            self.adapter
                .build_command(template_dir, &["clone", REPOSITORY, "."])
        };

        self.adapter.execute(command)?;
        Ok(())
    }

    pub fn list(&self, template_manifest: &Path) -> Result<Vec<Template>> {
        let template_manifest = std::fs::File::open(template_manifest)?;
        let template_manifest: TemplateManifest = serde_yaml::from_reader(template_manifest)?;
        Ok(template_manifest.templates)
    }
}
