pub mod item;

use axum::Router;
use item::item_routes;

pub fn all_routes() -> Router {
    item_routes()
}
