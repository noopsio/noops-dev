use crate::{bindgen, database::Database, schemas};
use axum::{
    extract::{DefaultBodyLimit, Json, Path, State},
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;

const MAX_CONTENT_SIZE_IN_BYTES: usize = 10_000_000;

pub fn create_routes(database: Arc<Database>) -> Router {
    Router::new()
        .route(
            "/api/:project_name/:function_name",
            post(create_function).delete(delete_function),
        )
        .with_state(database)
        .layer(DefaultBodyLimit::max(MAX_CONTENT_SIZE_IN_BYTES))
}

async fn create_function(
    Path((project_name, function_name)): Path<(String, String)>,
    State(database): State<Arc<Database>>,
    Json(body): Json<schemas::CreateFunctionSchema>,
) -> StatusCode {
    if !database.project_exists(&project_name).unwrap() {
        return StatusCode::NOT_FOUND;
    }

    let mut function = body;
    match bindgen::create_component(&function.wasm) {
        Ok(component) => function.wasm = component,
        Err(err) => {
            tracing::error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    match database.function_create(&project_name, &function_name, &function) {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            tracing::error!("{}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn delete_function(
    Path((project_name, function_name)): Path<(String, String)>,
    State(database): State<Arc<Database>>,
) -> StatusCode {
    if !database
        .function_exists(&project_name, &function_name)
        .unwrap()
    {
        return StatusCode::NOT_FOUND;
    }
    match database.function_delete(&project_name, &function_name) {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            tracing::error!("{}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use axum::{
        body::Body,
        http::{header, method::Method, Request},
    };
    use hyper;
    use lazy_static::lazy_static;
    use mime;
    use serde_json;
    use tempfile::tempdir;
    use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot nad ready`

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref FUNCTION_SCHEMA: schemas::CreateFunctionSchema = schemas::CreateFunctionSchema {
            project: PROJECT_NAME.to_string(),
            name: FUNCTION_NAME.to_string(),
            wasm: std::fs::read(env!("CARGO_CDYLIB_FILE_RETURN_STATUS_CODE_200"))
                .expect("Unable to read test module"),
            params: Default::default(),
        };
    }

    #[tokio::test]
    async fn create_function_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME).unwrap();
        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_string(&FUNCTION_SCHEMA.to_owned()).unwrap(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn create_function_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database);

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_string(&FUNCTION_SCHEMA.to_owned()).unwrap(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::NOT_FOUND, response.status());
    }

    #[tokio::test]
    async fn delete_function_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME).unwrap();
        database
            .function_create(
                PROJECT_NAME,
                FUNCTION_NAME,
                &schemas::CreateFunctionSchema {
                    name: FUNCTION_NAME.to_string(),
                    project: PROJECT_NAME.to_string(),
                    ..Default::default()
                },
            )
            .unwrap();

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn delete_function_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME).unwrap();

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::NOT_FOUND, response.status());
    }

    #[tokio::test]
    async fn delete_function_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database.clone());

        let uri = format!("/api/{}/{}", PROJECT_NAME, FUNCTION_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::NOT_FOUND, response.status());
    }
}
