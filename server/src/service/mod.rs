use common::dtos::GetFunctionDTO;

use crate::repository::function::Function;

pub mod auth;
pub mod function;
pub mod project;

const URL: &str = "http://localhost:8080/";

fn function_url(function_id: &str) -> String {
    URL.to_string() + function_id
}

impl From<Function> for GetFunctionDTO {
    fn from(value: Function) -> Self {
        GetFunctionDTO {
            name: value.name,
            language: value.language,
            hash: value.hash,
            link: function_url(&value.id),
        }
    }
}
