pub mod nodes;

use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/fetch", get(nodes::fetch_nodes))
        .route("/save", get(nodes::save_nodes))
        .with_state(pool)
}
