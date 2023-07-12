mod auth;
mod execute;
mod functions;
mod project;

use axum::{middleware, Router};
use std::sync::Arc;

use crate::database::{wasmstore::Wasmstore, Database};

pub fn create_routes(wasmstore: Arc<Wasmstore>, database: Arc<Database>) -> Router {
    Router::new()
        .merge(project::create_routes(wasmstore.clone()))
        .merge(functions::create_routes(wasmstore.clone()))
        .route_layer(middleware::from_fn_with_state(
            database.clone(),
            auth::auth_middleware,
        ))
        .merge(auth::create_routes(database))
        .merge(execute::create_routes(wasmstore))
}
