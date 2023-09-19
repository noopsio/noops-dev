use common::dtos::GetHandlerDTO;

use crate::repository::handler::Handler;

pub mod auth;
pub mod handler;
pub mod project;

const URL: &str = "https://app.noops.rocks";

fn handler_url(handler_id: &str) -> String {
    URL.to_string() + handler_id
}

impl From<Handler> for GetHandlerDTO {
    fn from(value: Handler) -> Self {
        GetHandlerDTO {
            name: value.name,
            language: value.language,
            hash: value.hash,
            link: handler_url(&value.id),
        }
    }
}
