use std::env;

use mongodb::Client;
use rocket::fairing::AdHoc;

const DEFAULT_MONGO_URL: &'static str = "mongodb://root:root@localhost:27017";
const DEFAULT_DB_NAME: &'static str = "test";

pub async fn get_db(db_name: String) -> mongodb::error::Result<mongodb::Database> {
    let db_url = env::var("MONGO_URL").unwrap_or(DEFAULT_MONGO_URL.to_string());

    let client_options = mongodb::options::ClientOptions::parse(db_url).await?;
    let client = Client::with_options(client_options)?;

    return Ok(client.database(&db_name));
}

pub fn connect() -> AdHoc {
    AdHoc::on_ignite("Connecting to Mongo", |rocket| async {
        let db_name = env::var("MONGO_DB_NAME").unwrap_or(DEFAULT_DB_NAME.to_string());

        match get_db(db_name).await {
            Ok(db) => rocket.manage(db),
            Err(e) => panic!("Failed to connect to Mongo: {}", e),
        }
    })
}

pub fn get_collection<T>(
    db: &mongodb::Database,
    collection_name: String,
) -> mongodb::Collection<T> {
    db.collection::<T>(&collection_name)
}
