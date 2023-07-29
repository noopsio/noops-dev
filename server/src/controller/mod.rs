// https://docs.rs/axum/0.6.10/axum/extract/struct.State.html#substates

mod auth;
mod execute;
mod function;
mod project;

use crate::service::auth::AuthService;
use crate::service::function::FunctionService;
<<<<<<< HEAD
use crate::service::{project::ProjectService, user::UserService};
=======
use crate::service::project::ProjectService;
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
use crate::wasmstore::WasmStore;
use axum::{extract::FromRef, middleware, Router};

#[derive(Debug, Clone)]
pub struct AppState {
    auth: AuthService,
<<<<<<< HEAD
    users: UserService,
=======
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
    projects: ProjectService,
    functions: FunctionService,
    wasmstore: WasmStore,
}

impl AppState {
    pub fn new(
        auth: AuthService,
<<<<<<< HEAD
        users: UserService,
=======
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
        projects: ProjectService,
        functions: FunctionService,
        wasmstore: WasmStore,
    ) -> Self {
        Self {
            auth,
<<<<<<< HEAD
            users,
=======
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
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

<<<<<<< HEAD
impl FromRef<AppState> for UserService {
    fn from_ref(app_state: &AppState) -> UserService {
        app_state.users.clone()
    }
}

=======
>>>>>>> 39b86c3 (feat: Consolidate cli commands into subcommands (#166))
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

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .merge(project::create_routes(state.clone()))
        .merge(function::create_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ))
        .merge(auth::create_routes(state.clone()))
        .merge(execute::create_routes(state))
}
