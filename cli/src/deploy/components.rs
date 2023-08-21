use crate::manifest::Component;
use common::dtos::{CreateFunctionDTO, GetFunctionDTO, Language};
use std::{fs, hash::Hash};

#[derive(Debug, Clone, Default, Eq, PartialOrd, Ord)]
pub struct BuildedComponent {
    pub name: String,
    pub language: Language,
    pub hash: String,
    pub wasm: Option<Vec<u8>>,
}

impl Hash for BuildedComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for BuildedComponent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl TryFrom<Component> for BuildedComponent {
    type Error = anyhow::Error;

    fn try_from(value: Component) -> Result<Self, Self::Error> {
        let wasm = fs::read(value.handler_path())?;
        let hash = common::hash::hash(&wasm);

        let component_with_payload = Self {
            name: value.name,
            language: value.language,
            hash,
            wasm: Some(wasm),
        };
        Ok(component_with_payload)
    }
}

impl From<BuildedComponent> for CreateFunctionDTO {
    fn from(value: BuildedComponent) -> Self {
        Self {
            name: value.name,
            language: value.language,
            wasm: value.wasm.unwrap(),
        }
    }
}

impl From<GetFunctionDTO> for BuildedComponent {
    fn from(value: GetFunctionDTO) -> Self {
        Self {
            name: value.name,
            language: value.language,
            hash: value.hash,
            wasm: Default::default(),
        }
    }
}
