use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, Client, Database};

use crate::settings::SETTINGS;

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> =
        AsyncOnce::new(async { get_database().await.unwrap() });
}

async fn get_database() -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(&SETTINGS.mongodb.connection_string).await?;
    let client = Client::with_options(client_options)?;

    Ok(client.database(&SETTINGS.mongodb.database))
}
