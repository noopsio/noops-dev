mod bindgen;
mod database;
mod errors;
mod executor;
mod github;
mod jwt;
mod routes;

use axum::Server;
use database::Database;
use std::{net::SocketAddr, path::Path};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

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

    let wasmstore = database::wasmstore::WasmStore::new(Path::new(WASMSTORE_PREFIX))?;
    let database = Database::new(Path::new(DATABASE_CONNECTION));

    let app = routes::create_routes(database, wasmstore).layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
