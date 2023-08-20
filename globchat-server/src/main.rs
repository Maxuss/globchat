pub mod routes;
pub mod db;
pub mod state;
pub mod response;
pub mod model;

use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::prelude::*;
use crate::db::mongo_connect;
use crate::routes::launch_api;

fn prepare_logging() -> anyhow::Result<()> {
    let stdout_log = tracing_subscriber::fmt::layer().compact();

    tracing_subscriber::registry()
        .with(
            stdout_log
                .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG)
                .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
                    metadata
                        .module_path()
                        .unwrap_or("unknown")
                        .starts_with("globchat_server")
                })),
        )
        .init();

    Ok(())
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    prepare_logging()?;

    info!("Initializing globchat...");

    dotenv()?;
    let db = mongo_connect(&std::env::var("MONGODB_PASSWORD")?).await?;

    info!("Launching on localhost:14670");
    launch_api(db).await
}
