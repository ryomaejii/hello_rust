pub mod list;

use axum::Router;
use list::list_routes;

pub fn all_routes() -> Router {
    list_routes()
}
