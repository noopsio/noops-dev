use poem_openapi::{ApiResponse, Object};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Object)]
pub struct CreateFunctionSchema {
    pub name: String,
    pub params: Vec<String>,
    pub project: String,
    pub wasm: Vec<u8>,
}

#[derive(ApiResponse, PartialEq, Debug)]
pub enum CreateResponse {
    /// Returned if the creation was successful
    #[oai(status = 204)]
    Ok,
    /// Returned if the resource already exists
    #[oai(status = 409)]
    Conflict,
    /// Returned if there was a critical server error
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse, PartialEq, Debug)]
pub enum CreateFunctionResponse {
    /// Returned if the function was creation successfully
    #[oai(status = 204)]
    Ok,
    /// Returned if the project is not found
    #[oai(status = 404)]
    NotFound,
    /// Returned if there was a critical server error
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse, PartialEq, Debug)]
pub enum DeleteResponse {
    /// Returned if the deletion was successful
    #[oai(status = 200)]
    Ok,
    /// Returned if the resource was not found
    #[oai(status = 404)]
    NotFound,
    /// Returned if there was a critical server error
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse,PartialEq, Debug)]
pub enum ExecuteResponse {
    /// Returned if the execution was successful
    #[oai(status = 200)]
    Ok,
    /// Returned if the resource was not found
    #[oai(status = 404)]
    NotFound,
    /// Returned if there was a critical server error
    #[oai(status = 500)]
    InternalServerError,
}
