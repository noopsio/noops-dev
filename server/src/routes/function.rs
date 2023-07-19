use crate::{bindgen, database::models::User, errors::Error};
use axum::{
    extract::{DefaultBodyLimit, Json, Path, State},
    http::StatusCode,
    routing::put,
    Extension, Router,
};

use super::AppState;

const MAX_CONTENT_SIZE_IN_BYTES: usize = 10_000_000;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/:project_name/:function_name",
            put(create_function).delete(delete_function),
        )
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_CONTENT_SIZE_IN_BYTES))
}

async fn create_function(
    Path((project_name, function_name)): Path<(String, String)>,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(function_dto): Json<dtos::CreateFunctionDTO>,
) -> Result<StatusCode, Error> {
    let wasm = bindgen::create_component(&function_dto.wasm)?;

    let function = state
        .database
        .create_function(&user.id, &project_name, function_name, &wasm)?;

    state
        .wasmstore
        .create_function(&user.id, &function.project_id, &function.id, &wasm)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_function(
    Path((project_name, function_name)): Path<(String, String)>,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<StatusCode, Error> {
    let function = state
        .database
        .delete_function(&user.id, &project_name, &function_name)?;

    state
        .wasmstore
        .delete_function(&user.id, &function.project_id, &function.id)?;

    Ok(StatusCode::NO_CONTENT)
}

/*

#[cfg(test)]
mod tests {

    use super::*;

    use axum::{
        body::Body,
        http::{header, method::Method, Request},
    };
    use dtos::CreateFunctionDTO;
    use lazy_static::lazy_static;
    use tempfile::tempdir;
    use tower::ServiceExt; // for `oneshot nad ready`

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref WASM: Vec<u8> =
        std::fs::read(env!("CARGO_CDYLIB_FILE_RETURN_STATUS_CODE_200")).unwrap();
        static ref FUNCTION: Function = Function {
            project: PROJECT_NAME.to_string(),
            name: FUNCTION_NAME.to_string(),
            component: bindgen::create_component(&WASM).unwrap(),
            hash: Default::default()
        };
        static ref CREATE_FUNCTION_DTO: CreateFunctionDTO = CreateFunctionDTO {
            wasm: std::fs::read(env!("CARGO_CDYLIB_FILE_RETURN_STATUS_CODE_200"))
            .expect("Unable to read test module")
        };
    }

    #[tokio::test]
    async fn create_function_ok() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME)?;
        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
        .uri(uri)
        .method(Method::POST)
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_string(
            &CREATE_FUNCTION_DTO.clone(),
        )?))?;

        let response = app.oneshot(request).await?;
        assert_eq!(StatusCode::NO_CONTENT, response.status());
        Ok(())
    }

    #[tokio::test]
    async fn create_function_project_not_found() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database);

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
        .uri(uri)
        .method(Method::POST)
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_string(
            &CREATE_FUNCTION_DTO.clone(),
        )?))?;

        let response = app.oneshot(request).await?;
        assert_eq!(StatusCode::NOT_FOUND, response.status());
        Ok(())
    }

    #[tokio::test]
    async fn delete_function_ok() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME)?;
        database.function_create(&FUNCTION)?;

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
        .uri(uri)
        .method(Method::DELETE)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(StatusCode::NO_CONTENT, response.status());
    Ok(())
}

#[tokio::test]
async fn delete_function_not_found() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
    let app = create_routes(database.clone());

    database.project_create(PROJECT_NAME)?;

    let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
    let request = Request::builder()
    .uri(uri)
    .method(Method::DELETE)
    .body(Body::empty())?;

let response = app.oneshot(request).await?;
assert_eq!(StatusCode::NOT_FOUND, response.status());
Ok(())
}

#[tokio::test]
async fn delete_function_project_not_found() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database.clone());

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
        .uri(uri)
        .method(Method::DELETE)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
    Ok(())
}
}

*/
