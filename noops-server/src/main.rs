mod bindgen;
mod database;
mod executor;
mod handler;
mod schemas;

use std::sync::Arc;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use poem::{
    listener::TcpListener,
    middleware::{AddData, Tracing},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;

const DATABASE_PATH: &str = "noops.db";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let database = Arc::new(database::Database::new(DATABASE_PATH)?);

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
