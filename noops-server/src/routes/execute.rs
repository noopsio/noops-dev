use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{bindgen, database::Database, executor};

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
) -> Result<String, StatusCode> {
    if !database
        .function_exists(&project_name, &function_name)
        .unwrap()
    {
        return Err(StatusCode::NOT_FOUND);
    }

    match database.function_get(&project_name, &function_name) {
        Ok(function) => {
            let mut query_list: Vec<(&str, &str)> = Vec::new();
            for (key, value) in query_map.iter() {
                query_list.push((key, value));
            }
            let result = query_list;

            let request = bindgen::Request {
                query_params: &result[..],
            };
            let response = executor::execute(function.wasm, request).await.unwrap();
            Ok("Ok".to_string())
        }
        Err(err) => {
            tracing::error!("{}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
