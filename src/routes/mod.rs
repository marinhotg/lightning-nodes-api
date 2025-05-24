mod get_json;
mod hello_world;
mod mirror_body_json;
mod mirror_body_string;

use axum::{
    Router,
    routing::{get, post},
};

use get_json::get_json;
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/get_json", get(get_json))
}
