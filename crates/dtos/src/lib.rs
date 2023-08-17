use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct CreateFunctionDTO {
    pub wasm: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GetProjectDTO {
    pub name: String,
    pub functions: Vec<GetFunctionDTO>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GetFunctionDTO {
    pub id: String,
    pub name: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GetJWTDTO {
    pub jwt: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct ErrorDTO {
    pub error_message: String,
}

impl ErrorDTO {
    pub fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}
