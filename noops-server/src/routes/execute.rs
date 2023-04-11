use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{bindgen, database::Database, executor};

use super::errors::AppError;

pub fn create_routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/:project_name/:function_name", get(execute))
        .with_state(database)
}

async fn execute(
    Path(project_name): Path<String>,
    Path(function_name): Path<String>,
    Query(query_map): Query<HashMap<String, String>>,
    State(database): State<Arc<Database>>,
) -> Result<Response, AppError> {
    if !database.function_exists(&project_name, &function_name)? {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let function = database.function_get(&project_name, &function_name)?;
    let mut query_list: Vec<(&str, &str)> = Vec::new();
    for (key, value) in query_map.iter() {
        query_list.push((key, value));
    }
    let result = query_list;

    let request = bindgen::Request {
        query_params: &result[..],
    };
    let _response = executor::execute(function.wasm, request).await?;
    Ok("Ok".to_string().into_response())
}
