use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
}