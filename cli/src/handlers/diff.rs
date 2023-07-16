use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

use crate::modules::Module;
use dtos::GetFunctionDTO;

type Update = (String, Vec<u8>);
type Create = (String, Vec<u8>);
type Remove = String;
type NotBuild = String;

#[derive(Default, Debug)]
pub struct ModuleDiff {
    pub create: Vec<Create>,
    pub update: Vec<Update>,
    pub remove: Vec<Remove>,
    pub not_build: Vec<Remove>,
}

impl ModuleDiff {
    pub fn new(
        project_name: &str,
        local_modules: &[Module],
        remote_modules: &[GetFunctionDTO],
    ) -> anyhow::Result<Self> {
        let (create, update, not_build) =
            Self::create_and_update(project_name, local_modules, remote_modules)?;
        let remove = Self::remove(local_modules, remote_modules)?;
        Ok(Self {
            create,
            update,
            remove,
            not_build,
        })
    }

    fn create_and_update(
        project_name: &str,
        local_modules: &[Module],
        remote_modules: &[GetFunctionDTO],
    ) -> anyhow::Result<(Vec<Create>, Vec<Update>, Vec<NotBuild>)> {
        let mut create: Vec<Create> = Default::default();
        let mut update: Vec<Update> = Default::default();
        let mut not_build: Vec<NotBuild> = Default::default();

        for local_module in local_modules {
            let remote_module = remote_modules
                .iter()
                .find(|&remote_module| remote_module.name == local_module.name);

            let module_out_path = Path::new(&local_module.name)
                .join("out")
                .join("handler.wasm");

            if !module_out_path.exists() {
                not_build.push(local_module.name.clone());
            } else {
                let wasm = std::fs::read(module_out_path)?;
                match remote_module {
                    Some(remote_module) => {
                        if remote_module.hash != Self::hash(&wasm) {
                            update.push((local_module.name.clone(), wasm));
                        }
                    }
                    None => create.push((local_module.name.clone(), wasm)),
                }
            }
        }
        Ok((create, update, not_build))
    }

    fn remove(
        local_modules: &[Module],
        remote_modules: &[GetFunctionDTO],
    ) -> anyhow::Result<Vec<Remove>> {
        let mut remove: Vec<Remove> = Default::default();

        for remote_module in remote_modules {
            let module_remove = local_modules
                .iter()
                .find(|&local_module| remote_module.name == local_module.name);

            if module_remove.is_none() {
                remove.push(remote_module.name.clone());
            }
        }

        Ok(remove)
    }

    fn hash(wasm: &[u8]) -> String {
        let mut hasher = DefaultHasher::new();
        wasm.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn has_changes(&self) -> bool {
        !(self.create.is_empty() && self.update.is_empty() && self.remove.is_empty())
    }

    pub fn has_not_builds(&self) -> bool {
        !self.not_build.is_empty()
    }
}
