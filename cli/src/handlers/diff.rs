use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

use crate::{filesystem, modules::Module};
use dtos::GetFunctionDTO;

type Update = (String, Vec<u8>);
type Create = (String, Vec<u8>);
type Remove = String;

#[derive(Default, Debug)]
pub struct ModuleDiff {
    pub create: Vec<Create>,
    pub update: Vec<Update>,
    pub remove: Vec<Remove>,
}

impl ModuleDiff {
    pub fn new(
        project_name: &str,
        local_modules: &[Module],
        remote_modules: &[GetFunctionDTO],
    ) -> anyhow::Result<Self> {
        let (create, update) =
            Self::create_and_update(project_name, local_modules, remote_modules)?;
        let remove = Self::remove(local_modules, remote_modules)?;
        Ok(Self {
            create,
            update,
            remove,
        })
    }

    fn create_and_update(
        project_name: &str,
        local_modules: &[Module],
        remote_modules: &[GetFunctionDTO],
    ) -> anyhow::Result<(Vec<Create>, Vec<Update>)> {
        let mut create: Vec<Create> = Default::default();
        let mut update: Vec<Update> = Default::default();

        for local_module in local_modules {
            let remote_module = remote_modules
                .iter()
                .find(|&remote_module| remote_module.name == local_module.name);

            let module_out_path = Path::new(&local_module.name).join("out");
            let module_path = filesystem::find_wasm(module_out_path).unwrap();
            let wasm = filesystem::read_wasm(&module_path)?;

            match remote_module {
                Some(remote_module) => {
                    if remote_module.hash != Self::hash(project_name, &local_module.name, &wasm) {
                        update.push((local_module.name.clone(), wasm));
                    }
                }
                None => create.push((local_module.name.clone(), wasm)),
            }
        }
        Ok((create, update))
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

    fn hash(project_name: &str, module_name: &str, wasm: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        project_name.hash(&mut hasher);
        module_name.hash(&mut hasher);
        wasm.hash(&mut hasher);
        hasher.finish()
    }

    pub fn has_changes(&self) -> bool {
        !(self.create.is_empty() && self.update.is_empty() && self.remove.is_empty())
    }
}
