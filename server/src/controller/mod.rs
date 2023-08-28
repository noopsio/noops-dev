// https://docs.rs/axum/0.6.10/axum/extract/struct.State.html#substates

mod auth;
mod execute;
mod function;
mod project;

use crate::service::auth::AuthService;
use crate::service::function::FunctionService;
use crate::service::project::ProjectService;
use crate::wasmstore::WasmStore;
use axum::{extract::FromRef, middleware, Router};

#[derive(Debug, Clone)]
pub struct AppState {
    auth: AuthService,
    projects: ProjectService,
    functions: FunctionService,
    wasmstore: WasmStore,
}

impl AppState {
    pub fn new(
        auth: AuthService,
        projects: ProjectService,
        functions: FunctionService,
        wasmstore: WasmStore,
    ) -> Self {
        Self {
            auth,
            projects,
            functions,
            wasmstore,
        }
    }
}

impl FromRef<AppState> for WasmStore {
    fn from_ref(app_state: &AppState) -> WasmStore {
        app_state.wasmstore.clone()
    }
}

impl FromRef<AppState> for AuthService {
    fn from_ref(app_state: &AppState) -> AuthService {
        app_state.auth.clone()
    }
}

impl FromRef<AppState> for ProjectService {
    fn from_ref(app_state: &AppState) -> ProjectService {
        app_state.projects.clone()
    }
}

impl FromRef<AppState> for FunctionService {
    fn from_ref(app_state: &AppState) -> FunctionService {
        app_state.functions.clone()
    }
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .merge(project::routes(state.clone()))
        .merge(function::routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ))
        .merge(auth::routes(state.clone()))
        .merge(execute::routes(state))
}
