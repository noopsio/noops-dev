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
    Path((project_name, function_name)): Path<(String, String)>,
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
    let response = executor::execute(function.wasm, request).await?;

    Ok((StatusCode::from_u16(response.status)?, response.body).into_response())
}

#[cfg(test)]
mod tests {

    use crate::database::Function;

    use super::*;
    use axum::{
        body::Body,
        http::{method::Method, Request},
    };
    use lazy_static::lazy_static;
    use tempfile::tempdir;
    use tower::ServiceExt; // for `oneshot nad ready`

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref WASM_RETURN_STATUS_CODE_200: Vec<u8> =
            std::fs::read(env!("CARGO_CDYLIB_FILE_RETURN_STATUS_CODE_200")).unwrap();
        static ref WASM_RETURN_PARAMS: Vec<u8> =
            std::fs::read(env!("CARGO_CDYLIB_FILE_RETURN_PARAMS")).unwrap();
        static ref RETURN_STATUS_CODE: Function = Function::new(
            PROJECT_NAME,
            FUNCTION_NAME,
            &bindgen::create_component(&WASM_RETURN_STATUS_CODE_200).unwrap()
        );
        static ref RETURN_PARAMS: Function = Function::new(
            PROJECT_NAME,
            FUNCTION_NAME,
            &bindgen::create_component(&WASM_RETURN_STATUS_CODE_200).unwrap()
        );
    }

    #[tokio::test]
    async fn return_status_code() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database.clone());

        let request = Request::builder()
            .uri(format!("/{}/{}", PROJECT_NAME, FUNCTION_NAME))
            .method(Method::GET)
            .body(Body::empty())?;

        database.project_create(PROJECT_NAME)?;
        database.function_create(&RETURN_STATUS_CODE)?;

        let response = app.oneshot(request).await?;
        assert_eq!(StatusCode::OK, response.status());
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn return_params() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database.clone());

        let request = Request::builder()
            .uri(format!(
                "/{}/{}?key1=value1&key2=value2&key3=value3",
                PROJECT_NAME, FUNCTION_NAME
            ))
            .method(Method::GET)
            .body(Body::empty())?;

        database.project_create(PROJECT_NAME)?;
        database.function_create(&RETURN_PARAMS)?;

        let response = app.oneshot(request).await?;
        let status = response.status();
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let body = String::from_utf8_lossy(&body);

        assert_eq!(StatusCode::OK, status);
        assert_eq!(
            format!("key1=value1\nkey2=value2\nkey3=value3\n"),
            body.to_owned()
        );
        Ok(())
    }
}
