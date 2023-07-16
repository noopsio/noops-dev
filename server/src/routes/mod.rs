// https://docs.rs/axum/0.6.10/axum/extract/struct.State.html#substates

mod auth;
mod execute;
mod function;
mod project;

use crate::database::{wasmstore::WasmStore, Database};
use axum::{extract::FromRef, middleware, Router};

#[derive(Debug, Clone)]
pub struct AppState {
    database: Database,
    wasmstore: WasmStore,
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.database.clone()
    }
}

pub fn create_routes(database: Database, wasmstore: WasmStore) -> Router {
    let state = AppState {
        database: database.clone(),
        wasmstore,
    };

    Router::new()
        .merge(project::create_routes(state.clone()))
        .merge(function::create_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.database.clone(),
            auth::auth_middleware,
        ))
        .merge(auth::create_routes(state.clone()))
        .merge(execute::create_routes(state))
}
