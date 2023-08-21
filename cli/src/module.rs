use common::dtos::{GetFunctionDTO, Language};
use common::hash;
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::templates::Template;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FunctionMetadata {
    pub name: String,
    pub language: Language,
}

impl From<FunctionMetadata> for BuildFunctionMetadata {
    fn from(value: FunctionMetadata) -> Self {
        Self {
            name: value.name.clone(),
            language: value.language,
            hash: hash::hash(&wasm(&value.name).unwrap()),
        }
    }
}

impl FunctionMetadata {
    pub fn from_template(template: &Template) -> Self {
        Self {
            name: template.name.clone(),
            language: template.language,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialOrd, Ord, Eq)]
pub struct BuildFunctionMetadata {
    pub name: String,
    pub language: Language,
    pub hash: String,
}

impl Hash for BuildFunctionMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for BuildFunctionMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl From<GetFunctionDTO> for BuildFunctionMetadata {
    fn from(value: GetFunctionDTO) -> Self {
        Self {
            name: value.name,
            language: value.language,
            hash: value.hash,
        }
    }
}

pub fn wasm(name: &str) -> anyhow::Result<Vec<u8>> {
    let path = Path::new(&name).join("out").join("handler.wasm");
    Ok(fs::read(path)?)
}
