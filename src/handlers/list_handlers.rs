use crate::models::list::{Item, ListResponse};
use axum::{
    extract::{Json, Path},
    response::IntoResponse,
};

pub async fn get_list(Path(list_id): Path<String>) -> impl IntoResponse {
    // ダミーデータを作成
    let response = ListResponse {
        title: format!("List {}", list_id),
        item: vec![
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
