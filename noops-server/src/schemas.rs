use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct CreateFunctionSchema {
    pub name: String,
    pub params: Vec<String>,
    pub project: String,
    pub wasm: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GetFunctionSchema {
    pub name: String,
    pub params: Vec<String>,
    pub project: String,
}
