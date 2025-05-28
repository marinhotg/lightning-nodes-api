use crate::services::api_client;
use crate::{models::node::NodeResponse, repositories::node_repository};
use axum::extract::Query;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::PgPool;

pub async fn fetch_nodes() -> Result<String, StatusCode> {
    let nodes = api_client::fetch_nodes().await?;
    Ok(format!("Fetched {} nodes", nodes.len()))
}

pub async fn save_nodes(State(pool): State<PgPool>) -> Result<String, StatusCode> {
    let nodes = api_client::fetch_nodes().await?;
    node_repository::save_nodes(&pool, nodes).await?;
    Ok("Nodes saved successfully".to_string())
}

pub async fn get_nodes(
    State(pool): State<PgPool>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<NodeResponse>>, StatusCode> {
    let nodes = node_repository::get_all_nodes(&pool, params.order).await?;
    Ok(Json(nodes))
}

#[derive(Debug, Deserialize)]
pub struct Params {
    order: Option<String>,
}
