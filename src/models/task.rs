use bson::{self, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    _id: ObjectId,
    pub task: String,
}

impl Model {
    pub fn to_bson(&self) -> bson::Document {
        doc! {
            "task": &self.task.to_owned(),
        }
    }
}
