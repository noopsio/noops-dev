use std::sync::Arc;

use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};

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
            Err(_) => schemas::CreateResponse::InternalServerError,
        }
    }

    /*
        #[oai(path = "/:project_name", method = "get")]
        async fn list(
            &self,
            project_name: Path<String>,
            database: Data<&Arc<Database>>,
        ) -> Result<Vec<FunctionDTO>> {
            if !database.project_exists(&project_name).unwrap() {
                return Err(Error::from_status(StatusCode::NOT_FOUND));
            }
            match database.project_list(&project_name) {
                Ok(functions) => todo!(),
                Err(_) => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
            }
        }
    */

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
            Err(_) => schemas::DeleteResponse::InternalServerError,
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
        match database.function_create(&project_name, &function_name, &body) {
            Ok(_) => schemas::CreateFunctionResponse::Ok,
            Err(_) => schemas::CreateFunctionResponse::InternalServerError,
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
        if !database.project_exists(&project_name).unwrap() {
            return schemas::DeleteResponse::NotFound;
        }
        match database.function_delete(&project_name, &function_name) {
            Ok(_) => schemas::DeleteResponse::Ok,
            Err(_) => schemas::DeleteResponse::InternalServerError,
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
                executor::execute(function.wasm).unwrap();
                schemas::ExecuteResponse::Ok
            }
            Err(_) => schemas::ExecuteResponse::InternalServerError,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::schemas::{CreateFunctionSchema, CreateResponse, DeleteResponse, CreateFunctionResponse};
    use super::*;
    use tempfile::tempdir;

    static DATABASE_NAME: &str = "noops.test_db";
    static PROJECT_NAME: &str = "test_project";
    static FUNCTION_NAME: &str = "test_function";


    #[tokio::test]
    async fn create_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());

        let api = API {};
        let project_name = PROJECT_NAME.to_string();

        let result = api
            .create_project(Path(project_name), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);
    }

    #[tokio::test]
    async fn create_project_conflict() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());


        let api = API {};
        let project_name = PROJECT_NAME.to_string();

        let result = api
            .create_project(Path(project_name.clone()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);
        let result = api
            .create_project(Path(project_name), Data(&database))
            .await;
        assert_eq!(CreateResponse::Conflict, result);
    }

    #[tokio::test]
    async fn delete_project_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());


        let api = API {};
        let project_name = PROJECT_NAME.to_string();

        let result = api
            .create_project(Path(project_name.clone()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);
        let result = api
            .delete_project(Path(project_name), Data(&database))
            .await;
        assert_eq!(DeleteResponse::Ok, result);
    }

    #[tokio::test]
    async fn delete_project_not_found() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());


        let api = API {};
        let project_name = PROJECT_NAME.to_string();

        let result = api
            .delete_project(Path(project_name), Data(&database))
            .await;
        assert_eq!(DeleteResponse::NotFound, result);
    }

    #[tokio::test]
    async fn create_function_ok() {
        let temp_dir = tempdir().unwrap();
        let database = Arc::new(Database::new(temp_dir.path().join(DATABASE_NAME)).unwrap());


        let api = API {};
        let project_name = PROJECT_NAME.to_string();
        let function_name = FUNCTION_NAME.to_string();
        let function_schema = CreateFunctionSchema {
            project: project_name.clone(),
            name: function_name.clone(),
            wasm: vec![0],
            params: vec!["".to_string()],
        };

        let result = api
            .create_project(Path(project_name.clone()), Data(&database))
            .await;
        assert_eq!(CreateResponse::Ok, result);

        let result = api
            .create_function(
                Path(project_name),
                Path(function_name),
                Json(function_schema),
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
        let project_name = PROJECT_NAME.to_string();
        let function_name = FUNCTION_NAME.to_string();
        let function_schema = CreateFunctionSchema {
            project: project_name.clone(),
            name: function_name.clone(),
            wasm: vec![0],
            params: vec!["".to_string()],
        };

        let result = api
            .create_function(
                Path(project_name),
                Path(function_name),
                Json(function_schema),
                Data(&database),
            )
            .await;
        assert_eq!(CreateFunctionResponse::NotFound, result);
    }
}
