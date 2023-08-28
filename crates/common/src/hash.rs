use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn hash(wasm: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    wasm.hash(&mut hasher);
    hasher.finish().to_string()
}
