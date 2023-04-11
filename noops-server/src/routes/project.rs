use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

use super::errors::AppError;
use crate::database::Database;

pub fn create_routes(database: Arc<Database>) -> Router {
    Router::new()
        .route(
            "/api/:project_name",
            get(list_project)
                .post(create_project)
                .delete(delete_project),
        )
        .with_state(database)
}

async fn create_project(
    Path(project_name): Path<String>,
    State(database): State<Arc<Database>>,
) -> Result<StatusCode, AppError> {
    if database.project_exists(&project_name).unwrap() {
        return Ok(StatusCode::CONFLICT);
    }

    database.project_create(&project_name)?;
    Ok(StatusCode::OK)
}
//axum::Json<Vec<schemas::GetFunctionSchema>
async fn list_project(
    Path(project_name): Path<String>,
    State(database): State<Arc<Database>>,
) -> Result<Response, AppError> {
    if !database.project_exists(&project_name).unwrap() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
    let functions = database.project_list(&project_name)?;
    Ok((StatusCode::OK, Json(functions)).into_response())
}

async fn delete_project(
    Path(project_name): Path<String>,
    State(database): State<Arc<Database>>,
) -> Result<StatusCode, AppError> {
    if !database.project_exists(&project_name).unwrap() {
        return Ok(StatusCode::NOT_FOUND);
    }
    database.project_delete(&project_name)?;
    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::schemas;
    use axum::{
        body::Body,
        http::{method::Method, Request},
    };
    use hyper;
    use serde_json;
    use tempfile::tempdir;
    use tower::ServiceExt; // for `oneshot nad ready`

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    #[tokio::test]
    async fn create_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn create_project_conflict() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME).unwrap();
        let uri = format!("/api/{}", PROJECT_NAME);

        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::CONFLICT, response.status());
    }

    #[tokio::test]
    #[ignore]
    async fn get_project_empty_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        let function_list: Vec<schemas::GetFunctionSchema> = Default::default();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<schemas::GetFunctionSchema> = serde_json::from_slice(&body).unwrap();
        // Check status
        assert_eq!(function_list, body);
    }

    #[tokio::test]
    async fn get_project_ok() {
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

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();

        let function_list: Vec<schemas::GetFunctionSchema> = vec![schemas::GetFunctionSchema {
            name: FUNCTION_NAME.to_string(),
            project: PROJECT_NAME.to_string(),
            ..Default::default()
        }];

        let response = app.oneshot(request).await.unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<schemas::GetFunctionSchema> = serde_json::from_slice(&body).unwrap();
        assert_eq!(function_list, body);
    }

    #[tokio::test]
    async fn get_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::NOT_FOUND, response.status());
    }

    #[tokio::test]
    async fn delete_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        database.project_create(PROJECT_NAME).unwrap();

        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn delete_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(StatusCode::NOT_FOUND, response.status());
    }
}
