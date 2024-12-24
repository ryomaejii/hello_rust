use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub checked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ItemList {
    pub title: String,
    pub items: Vec<Item>,
}
