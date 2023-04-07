use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use std::sync::Arc;
use tracing;

use crate::bindgen;
use crate::database::Database;
use crate::executor;
use crate::schemas;

pub struct API;

#[OpenApi]
impl API {
    /// Create a project
    #[oai(path = "/api/:project_name", method = "post")]
    async fn create_project(
        &self,
        project_name: Path<String>,
        database: Data<&Arc<Database>>,
    ) -> schemas::CreateResponse {
        if database.project_exists(&project_name).unwrap() {
            return schemas::CreateResponse::Conflict;
        }

        match database.project_create(&project_name) {
            Ok(_) => schemas::CreateResponse::Ok,
            Err(err) => {
                tracing::error!("{}", err);
                schemas::CreateResponse::InternalServerError
            }
        }
    }

    /// List all functions in a project
    #[oai(path = "/:project_name", method = "get")]
    async fn list(
        &self,
        project_name: Path<String>,
        database: Data<&Arc<Database>>,
    ) -> schemas::GetProjectResponse {
        if !database.project_exists(&project_name).unwrap() {
            return schemas::GetProjectResponse::NotFound;
        }
        match database.project_list(&project_name) {
            Ok(functions) => schemas::GetProjectResponse::Ok(Json(functions)),
            Err(err) => {
                tracing::error!("{}", err);
                schemas::GetProjectResponse::InternalServerError
            }
        }
    }

    /// Delete a project
    #[oai(path = "/api/:project_name", method = "delete")]
    async fn delete_project(
        &self,
        project_name: Path<String>,
        database: Data<&Arc<Database>>,
    ) -> schemas::DeleteResponse {
        if !database.project_exists(&project_name).unwrap() {
            return schemas::DeleteResponse::NotFound;
        }
        match database.project_delete(&project_name) {
            Ok(_) => schemas::DeleteResponse::Ok,
            Err(err) => {
                tracing::error!("{}", err);
                schemas::DeleteResponse::InternalServerError
            }
        }
    }

    /// Create a function
    #[oai(path = "/api/:project_name/:function_name", method = "post")]
    async fn create_function(
        &self,
        project_name: Path<String>,
        function_name: Path<String>,
        body: Json<schemas::CreateFunctionSchema>,
        database: Data<&Arc<Database>>,
    ) -> schemas::CreateFunctionResponse {
        if !database.project_exists(&project_name).unwrap() {
            return schemas::CreateFunctionResponse::NotFound;
        }

        let mut function = body.0;
        match bindgen::create_component(&function.wasm) {
            Ok(component) => function.wasm = component,
            Err(err) => {
                tracing::error!("{}", err);
                return schemas::CreateFunctionResponse::InternalServerError;
            }
        };

        match database.function_create(&project_name, &function_name, &function) {
            Ok(_) => schemas::CreateFunctionResponse::Ok,
            Err(err) => {
                tracing::error!("{}", err);
                schemas::CreateFunctionResponse::InternalServerError
            }
        }
    }

    /// Delete a function
    #[oai(path = "/api/:project_name/:function_name", method = "delete")]
    async fn delete_function(
        &self,
        project_name: Path<String>,
        function_name: Path<String>,
        database: Data<&Arc<Database>>,
    ) -> schemas::DeleteResponse {
        if !database
            .function_exists(&project_name, &function_name)
            .unwrap()
        {
            return schemas::DeleteResponse::NotFound;
        }
        match database.function_delete(&project_name, &function_name) {
            Ok(_) => schemas::DeleteResponse::Ok,
            Err(err) => {
                tracing::error!("{}", err);
                schemas::DeleteResponse::InternalServerError
            }
        }
    }

    /// Execute a function
    #[oai(path = "/:project_name/:function_name", method = "get")]
    async fn execute(
        &self,
        project_name: Path<String>,
        function_name: Path<String>,
        database: Data<&Arc<Database>>,
    ) -> schemas::ExecuteResponse {
        if !database
            .function_exists(&project_name, &function_name)
            .unwrap()
        {
            return schemas::ExecuteResponse::NotFound;
        }

        match database.function_get(&project_name, &function_name) {
            Ok(function) => {
                let request = bindgen::Request {
                    headers: &Vec::default(),
                    params: &Vec::default(),
                };

                let _response = executor::execute(function.wasm, request).await.unwrap();
                schemas::ExecuteResponse::Ok
            }
            Err(err) => {
                tracing::error!("{}", err);
                schemas::ExecuteResponse::InternalServerError
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::schemas::{
        CreateFunctionResponse, CreateFunctionSchema, CreateResponse, DeleteResponse,
        GetFunctionSchema, GetProjectResponse,
    };
    use lazy_static::lazy_static;
    use poem_openapi::payload::Json;
    use tempfile::tempdir;

    const DATABASE_NAME: &str = "noops.test_db";
    const PROJECT_NAME: &str = "test_project";
    const FUNCTION_NAME: &str = "test_function";

    lazy_static! {
        static ref FUNCTION_SCHEMA: CreateFunctionSchema = CreateFunctionSchema {
            project: PROJECT_NAME.to_string(),
            name: FUNCTION_NAME.to_string(),
            wasm: std::fs::read(env!("CARGO_CDYLIB_FILE_RETURN_STATUS_CODE_200"))
                .expect("Unable to read test module"),
            params: Vec::default(),
        };
    }

    #[tokio::test]
    async fn create_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);
    }

    #[tokio::test]
    async fn create_project_conflict() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Conflict, result);
    }

    #[tokio::test]
    async fn get_project_empty_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .list(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;

        let function_list: Vec<GetFunctionSchema> = vec![];

        assert_eq!(GetProjectResponse::Ok(Json(function_list)), result);
    }

    #[tokio::test]
    async fn get_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .create_function(
                Path(PROJECT_NAME.to_string()),
                Path(FUNCTION_NAME.to_string()),
                Json(FUNCTION_SCHEMA.clone()),
                Data(&database),
            )
            .await;
        assert_eq!(CreateFunctionResponse::Ok, result);

        let result = api
            .list(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;

        let function_list: Vec<GetFunctionSchema> = vec![GetFunctionSchema {
            name: FUNCTION_NAME.to_string(),
            project: PROJECT_NAME.to_string(),
            params: Vec::default(),
        }];

        assert_eq!(GetProjectResponse::Ok(Json(function_list)), result);
    }

    #[tokio::test]
    async fn get_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .list(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;

        assert_eq!(GetProjectResponse::NotFound, result);
    }

    #[tokio::test]
    async fn delete_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .delete_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(DeleteResponse::Ok, result);
    }

    #[tokio::test]
    async fn delete_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());

        let api = API {};
        let result = api
            .delete_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;

        assert_eq!(DeleteResponse::NotFound, result);
    }

    #[tokio::test]
    async fn create_function_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .create_function(
                Path(PROJECT_NAME.to_string()),
                Path(FUNCTION_NAME.to_string()),
                Json(FUNCTION_SCHEMA.clone()),
                Data(&database),
            )
            .await;
        assert_eq!(CreateFunctionResponse::Ok, result);
    }

    #[tokio::test]
    async fn create_function_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_function(
                Path(PROJECT_NAME.to_string()),
                Path(FUNCTION_NAME.to_string()),
                Json(FUNCTION_SCHEMA.clone()),
                Data(&database),
            )
            .await;
        assert_eq!(CreateFunctionResponse::NotFound, result);
    }

    #[tokio::test]
    async fn delete_function_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .create_function(
                Path(PROJECT_NAME.to_string()),
                Path(FUNCTION_NAME.to_string()),
                Json(FUNCTION_SCHEMA.clone()),
                Data(&database),
            )
            .await;
        assert_eq!(CreateFunctionResponse::Ok, result);

        let result = api
            .delete_function(
                Path(PROJECT_NAME.to_string()),
                Path(FUNCTION_NAME.to_string()),
                Data(&database),
            )
            .await;
        assert_eq!(DeleteResponse::Ok, result);
    }

    #[tokio::test]
    async fn delete_function_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .create_project(Path(PROJECT_NAME.to_string()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .delete_function(
                Path(PROJECT_NAME.to_string()),
                Path("unknown_function".to_string()),
                Data(&database),
            )
            .await;
        assert_eq!(DeleteResponse::NotFound, result);
    }

    #[tokio::test]
    async fn delete_function_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());
        let api = API {};

        let result = api
            .delete_function(
                Path(PROJECT_NAME.to_string()),
                Path(FUNCTION_NAME.to_string()),
                Data(&database),
            )
            .await;
        assert_eq!(DeleteResponse::NotFound, result);
    }
}
