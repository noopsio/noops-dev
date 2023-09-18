mod bindgen;
mod controller;
mod errors;
mod executor;
mod github;
mod jwt;
mod repository;
mod service;
mod wasmstore;

use crate::controller::AppState;
use axum::Server;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use github::GithubClient;
use service::{auth::AuthService, handler::HandlerService, project::ProjectService};
use std::{net::SocketAddr, path::Path};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const WASMSTORE_PREFIX: &str = "./wasmstore";
const DATABASE_CONNECTION: &str = "./noops.sqlite";
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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
    run_database_migration()?;
    let app = controller::routes(state).layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn create_app_state(database_path: &Path, wasmstore_path: &Path) -> anyhow::Result<AppState> {
    let (users, projects, handlers) = repository::new(database_path);
    let wasmstore = wasmstore::WasmStore::new(wasmstore_path)?;

    let auth_service = AuthService::new(GithubClient::new(), users);
    let project_service = ProjectService::new(projects.clone(), handlers.clone());
    let handler_service = HandlerService::new(projects, handlers, wasmstore.clone());

    let state = AppState::new(auth_service, project_service, handler_service, wasmstore);

    Ok(state)
}

fn run_database_migration() -> anyhow::Result<()> {
    tracing::info!("Running Database Migrations");
    let mut connection = SqliteConnection::establish(DATABASE_CONNECTION)?;
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    tracing::info!("Database Migrations Successful");
    Ok(())
}
