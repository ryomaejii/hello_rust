use crate::models::item::{Item, ItemList};
use axum::{
    extract::{Json, Path},
    response::IntoResponse,
};

pub async fn get_item_list(Path(item_id): Path<String>) -> impl IntoResponse {
    // ダミーデータを作成
    let response = ItemList {
        title: format!("List {}", item_id),
        items: vec![
            Item {
                id: "item1".to_string(),
                name: "Sample Item 1".to_string(),
                checked: false,
            },
            Item {
                id: "item2".to_string(),
                name: "Sample Item 2".to_string(),
                checked: true,
            },
        ],
    };

    // JSON レスポンスとして返す
    Json(response)
}
