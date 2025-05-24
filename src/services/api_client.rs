use crate::models::node::ApiNode;

pub async fn fetch_nodes() -> Vec<ApiNode> {
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let response = reqwest::get(url).await.unwrap();
    let text = response.text().await.unwrap();

    println!("Fetched {} bytes", text.len());

    serde_json::from_str(&text).unwrap()
}
