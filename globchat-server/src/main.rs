pub mod db;
pub mod routes;

use std::net::SocketAddr;

use clap::Parser;
use tracing::log::info;
use tracing_subscriber::prelude::*;

use crate::db::init_db;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MongoDB connection string
    #[arg(long, short, default_value = "mongodb://localhost:27017")]
    mongo_uri: String,

    /// MongoDB username
    #[arg(long, default_value = "globchat")]
    mongo_username: String,

    /// MongoDB username
    #[arg(long, default_value = "globchat")]
    mongo_password: String,

    /// Whether to use pretty logging instead of default compact
    #[arg(long, default_value_t = false)]
    pretty_logging: bool,
}

#[derive(Parser, Debug)]
struct MongoArgs {}

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.pretty_logging {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "globchat_server=debug,tower_http=warn".into()),
            )
            .with(tracing_subscriber::fmt::layer().pretty())
            .init()
    } else {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "globchat_server=info,tower_http=warn".into()),
            )
            .with(tracing_subscriber::fmt::layer().compact())
            .init()
    }

    let db = init_db(&args.mongo_uri, args.mongo_username, args.mongo_password).await?;

    let app = routes::route().with_state(db);

    let listener = std::net::TcpListener::bind("127.0.0.1:3000").unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
