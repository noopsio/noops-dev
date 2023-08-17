use super::AppState;
use crate::{
    errors::Error::{self},
    repository::user::User,
    service::project::ProjectService,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/:project_name",
            get(get_project).post(create_project).delete(delete_project),
        )
        .with_state(state)
}

async fn create_project(
    Path(project_name): Path<String>,
    State(projects): State<ProjectService>,
    Extension(user): Extension<User>,
) -> Result<StatusCode, Error> {
    projects.create(user.id, project_name)?;
    Ok(StatusCode::NO_CONTENT)
}
async fn get_project(
    Path(project_name): Path<String>,
    State(projects): State<ProjectService>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    let project = projects.read(&user, &project_name)?;
    Ok((StatusCode::OK, Json(project)))
}

async fn delete_project(
    Path(project_name): Path<String>,
    State(projects): State<ProjectService>,
    Extension(user): Extension<User>,
) -> Result<StatusCode, Error> {
    projects.delete(&user, &project_name)?;
    Ok(StatusCode::OK)
}

/*
#[cfg(test)]
mod tests {

    use crate::database::wasmstore::Function;

    use super::*;
    use axum::{
        body::Body,
        http::{method::Method, Request},
    };
    use dtos::GetFunctionDTO;
    use lazy_static::lazy_static;
    use tempfile::tempdir;
    use tower::ServiceExt; // for `oneshot and ready`

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref FUNCTION: Function = Function {
            name: FUNCTION_NAME.to_string(),
            project: PROJECT_NAME.to_string(),
            ..Default::default()
        };
    }



    #[tokio::test]
    async fn create_project_ok() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
        .uri(uri)
            .method(Method::POST)
            .body(Body::empty())?;
        let response = app.oneshot(request).await?;

        assert_eq!(StatusCode::NO_CONTENT, response.status());
        Ok(())
    }

    #[tokio::test]
    async fn create_project_conflict() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        database.project_create(PROJECT_NAME)?;
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);

        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .body(Body::empty())?;

        let response = app.oneshot(request).await?;
        assert_eq!(StatusCode::CONFLICT, response.status());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn get_project_empty_ok() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        database.project_create(PROJECT_NAME)?;
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .body(Body::empty())?;

        let function_list: Vec<dtos::GetFunctionDTO> = Default::default();
        let response = app.oneshot(request).await?;
        assert_eq!(StatusCode::OK, response.status());
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let body: Vec<dtos::GetFunctionDTO> = serde_json::from_slice(&body)?;
        assert_eq!(function_list, body);
        Ok(())
    }

    #[tokio::test]
    async fn get_project_ok() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database.clone());

        database.project_create(PROJECT_NAME)?;
        database.function_create(&FUNCTION)?;

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .body(Body::empty())?;

        let function_list: Vec<GetFunctionDTO> = vec![GetFunctionDTO {
            name: FUNCTION_NAME.to_string(),
            project: PROJECT_NAME.to_string(),
            ..Default::default()
        }];

        let response = app.oneshot(request).await?;
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let body: Vec<GetFunctionDTO> = serde_json::from_slice(&body)?;
        assert_eq!(function_list, body);
        Ok(())
    }

    #[tokio::test]
    async fn get_project_not_found() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .body(Body::empty())?;

        let response = app.oneshot(request).await?;
        assert_eq!(StatusCode::NOT_FOUND, response.status());
        Ok(())
    }

    #[tokio::test]
    async fn delete_project_ok() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        database.project_create(PROJECT_NAME)?;

        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
        let request = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .body(Body::empty())?;

        let response = app.oneshot(request).await?;

        assert_eq!(StatusCode::NO_CONTENT, response.status());
        Ok(())
    }

    #[tokio::test]
    async fn delete_project_not_found() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let database = Arc::new(Wasmstore::new(temp_dir.path().join(DATABASE_NAME))?);
        let app = create_routes(database);

        let uri = format!("/api/{}", PROJECT_NAME);
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
