mod auth;
mod execute;
mod functions;
mod project;

use axum::{middleware, Router};
use std::sync::Arc;

use crate::database::Database;

pub fn create_routes(database: Arc<Database>) -> Router {
    Router::new()
        .merge(project::create_routes(database.clone()))
        .merge(functions::create_routes(database.clone()))
        .route_layer(middleware::from_fn(auth::auth_middleware)) //This activates the auth middleware
        .merge(execute::create_routes(database))
        .merge(auth::create_routes())
}
