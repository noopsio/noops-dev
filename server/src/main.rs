mod bindgen;
mod controller;
mod errors;
mod executor;
mod github;
mod jwt;
mod repository;
mod service;
mod wasmstore;

use axum::Server;
use github::GithubClient;
use service::{auth::AuthService, function::FunctionService, project::ProjectService};
use std::{net::SocketAddr, path::Path};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

use crate::controller::AppState;

const WASMSTORE_PREFIX: &str = "./wasmstore";
const DATABASE_CONNECTION: &str = "./noops.sqlite";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "noops_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = create_app_state(Path::new(DATABASE_CONNECTION), Path::new(WASMSTORE_PREFIX))?;
    let app = controller::create_routes(state).layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn create_app_state(database_path: &Path, wasmstore_path: &Path) -> anyhow::Result<AppState> {
    let (users, projects, functions) = repository::new(database_path);
    let wasmstore = wasmstore::WasmStore::new(wasmstore_path)?;

    let auth_service = AuthService::new(GithubClient::new(), users);
    let project_service = ProjectService::new(projects.clone(), functions.clone());
    let function_service = FunctionService::new(projects, functions, wasmstore.clone());

    let state = AppState::new(auth_service, project_service, function_service, wasmstore);

    Ok(state)
}
