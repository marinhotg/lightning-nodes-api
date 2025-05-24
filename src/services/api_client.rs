pub async fn fetch_nodes() -> String {
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let response = reqwest::get(url).await.unwrap();
    response.text().await.unwrap()
}
