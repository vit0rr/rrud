// let mut changing_thing = true;
use mongodb::bson::{doc, Document};
use rocket::serde::Deserialize;
use serde_json::from_str;
use serde_json::json;

mod mongodb_utils;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct Task<'a> {
    task: &'a str,
}

#[post("/task", data = "<task>")]
async fn insert_task(task: String) -> String {
    // @TODO: implement db as param
    let client_options = mongodb_utils::mongodb_connection::get_connection().await;
    let db = mongodb_utils::mongodb_connection::get_database(&client_options, "ToDo").await;
    let collection = db.collection::<Document>("tasks");

    let serde_task: Task<'_> = from_str(&task).unwrap();
    let task = serde_task.task;

    collection
        .insert_one(
            doc! {
                "task": task,
            },
            None,
        )
        .await
        .unwrap();

    return json!({
        "status": "success",
        "message": "Task inserted successfully",
    })
    .to_string();
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![insert_task])
}
