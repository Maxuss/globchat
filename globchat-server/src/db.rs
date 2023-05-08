use mongodb::{
    options::{AuthMechanism, ClientOptions},
    Client, Database,
};
use tracing::log::info;

#[tracing::instrument]
pub async fn init_db(uri: &str) -> anyhow::Result<Database> {
    let mut options = ClientOptions::parse(uri).await?;

    options.credential = Some(
        mongodb::options::Credential::builder()
            .username("globchat".to_owned())
            .password("globchat".to_owned())
            .mechanism(AuthMechanism::ScramSha1)
            .source("globchat".to_owned())
            .build(),
    );

    let client = Client::with_options(options)?;

    let db = client.database("globchat");
    info!("Initialized MongoDB for Globchat");

    Ok(db)
}
