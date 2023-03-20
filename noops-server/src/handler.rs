use std::sync::Arc;

use poem::web::Data;
use poem_openapi::{
    param::Path,
    payload::Json,
    OpenApi,
};

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
    ) -> schemas::CreateResponse {
        if !database.project_exists(&project_name).unwrap() {
            return schemas::CreateResponse::Conflict;
        }
        match database.function_create(&project_name, &function_name, &body) {
            Ok(_) => schemas::CreateResponse::Ok,
            Err(_) => schemas::CreateResponse::InternalServerError,
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
            Err(_) => schemas::ExecuteResponse::InternalServerError
        }
    }
}
