use crate::services::api_client;

pub async fn fetch_nodes() -> String {
    api_client::fetch_nodes().await
}
