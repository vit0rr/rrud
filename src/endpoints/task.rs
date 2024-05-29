use mongodb::{bson::doc, Collection, Database};
use rocket::{futures::StreamExt, State};

use crate::{
    db,
    models::{self, task::Schema},
};

#[post("/task", data = "<body>")]
pub async fn post(db: &State<Database>, body: String) -> Result<String, String> {
    let collection = db::mongo::get_collection(db, "tasks".to_string());
    let task: models::task::Model = serde_json::from_str(&body).unwrap();

    let task = models::task::Model { task: task.task };

    match collection.insert_one(task.to_bson(), None).await {
        Ok(res) => Ok(res.inserted_id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/task")]
pub async fn get(db: &State<Database>) -> Result<String, String> {
    let collection = db::mongo::get_collection(db, "tasks".to_string());

    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(e) => return Err(e.to_string()),
    };

    let mut tasks: Vec<models::task::Schema> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                tasks.push(Schema::from(document));
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[get("/search_task", data = "<body>")]
pub async fn get_task(db: &State<Database>, body: String) -> Result<String, String> {
    let collection: Collection<Schema> = db::mongo::get_collection(db, "tasks".to_string());

    let task: models::task::Model = serde_json::from_str(&body).unwrap();

    let task = models::task::Model { task: task.task };

    let document = match collection.find_one(doc! { "task": task.task }, None).await {
        Ok(document) => document,
        Err(e) => return Err(e.to_string()),
    };

    match document {
        Some(document) => Ok(document.task),
        None => Err("Task not found".to_string()),
    }
}
