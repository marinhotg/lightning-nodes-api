use crate::services::api_client;
use crate::{models::node::NodeResponse, repositories::node_repository};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn fetch_nodes() -> String {
    let nodes = api_client::fetch_nodes().await;
    format!("Fetched {} nodes", nodes.len())
}

pub async fn save_nodes(State(pool): State<PgPool>) -> String {
    let nodes = api_client::fetch_nodes().await;
    node_repository::save_nodes(&pool, nodes.clone()).await;
    format!("{} Nodes saved successfully", nodes.len()).to_string()
}

pub async fn get_nodes(State(pool): State<PgPool>) -> Json<Vec<NodeResponse>> {
    let nodes = node_repository::get_all_nodes(&pool).await;
    Json(nodes)
}
