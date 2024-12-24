use crate::handlers::item_handlers::get_item_list;
use axum::{routing::get, Router};

pub fn item_routes() -> Router {
    Router::new().route("/items/:item_id", get(get_item_list))
}
