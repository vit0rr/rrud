pub mod mongodb_connection {
    use mongodb::{options::ClientOptions, Client};

    pub async fn get_connection() -> Client {
        let client_options = ClientOptions::parse("mongodb://root:root@localhost:27017")
            .await
            .unwrap();
        Client::with_options(client_options).unwrap()
    }

    pub async fn get_database(client: &Client, database_name: &str) -> mongodb::Database {
        client.database(database_name)
    }
}
