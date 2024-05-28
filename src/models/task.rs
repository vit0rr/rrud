use bson::{self, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

pub struct Model {
    pub task: String,
}

impl Model {
    pub fn to_bson(&self) -> bson::Document {
        doc! {
            "task": &self.task,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub _id: ObjectId,
    pub task: String,
}
