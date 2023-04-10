mod execute;
mod functions;
mod project;

use axum::Router;
use std::sync::Arc;

use crate::database::Database;

pub fn create_routes(database: Arc<Database>) -> Router {
    Router::new()
        .merge(project::create_routes(database.clone()))
        .merge(functions::create_routes(database.clone()))
        .merge(execute::create_routes(database))
}
