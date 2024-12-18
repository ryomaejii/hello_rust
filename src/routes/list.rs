use crate::handlers::list_handlers::get_list;
use axum::{routing::get, Router};

pub fn list_routes() -> Router {
    Router::new().route("/list/:list_id", get(get_list))
}
