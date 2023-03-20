mod database;
mod executor;
mod handler;
mod schemas;

use std::sync::Arc;
use tracing_subscriber;

use poem::{
    listener::TcpListener,
    middleware::{AddData, Tracing},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;

const DATABASE_PATH: &str = "noops.db";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    let database = Arc::new(database::Database::new(DATABASE_PATH)?);
    tracing_subscriber::fmt::init();

    let api = OpenApiService::new(handler::API, "noops API", "1.0");
    let doc = api.swagger_ui();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(
            Route::new()
                .nest("/", api)
                .nest("/doc", doc)
                .with(AddData::new(database))
                .with(Tracing::default()),
        )
        .await?;

    Ok(())
}
