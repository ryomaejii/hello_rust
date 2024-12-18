use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub checked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
    pub title: String,
    pub item: Vec<Item>,
}
