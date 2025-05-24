mod nodes;

use axum::{Router, routing::get};

use nodes::fetch_nodes;

pub fn app() -> Router {
    Router::new().route("/fetch", get(fetch_nodes))
}
