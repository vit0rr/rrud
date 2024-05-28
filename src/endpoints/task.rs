use mongodb::{bson::doc, Database};
use rocket::State;

use crate::{db, models};

#[post("/task", data = "<task>")]
pub async fn post(db: &State<Database>, task: String) -> Result<String, String> {
    let collection = db::mongo::get_collection(db, "tasks".to_string());

    let task = models::task::Model { task: task };

    match collection.insert_one(task.to_bson(), None).await {
        Ok(res) => Ok(res.inserted_id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
