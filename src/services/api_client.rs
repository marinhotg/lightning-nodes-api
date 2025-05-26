use crate::models::node::ApiNode;
use axum::http::StatusCode;

pub async fn fetch_nodes() -> Result<Vec<ApiNode>, StatusCode> {
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let response = reqwest::get(url).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let text = response.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    println!("Fetched {} bytes", text.len());

    serde_json::from_str(&text).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
