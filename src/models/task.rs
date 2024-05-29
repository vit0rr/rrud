use bson::{self, doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub task: String,
}

impl Model {
    pub fn to_bson(&self) -> bson::Document {
        doc! {
            "task": &self.task.to_owned(),
        }
    }
}
