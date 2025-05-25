use crate::repositories::node_repository;
use crate::services::api_client;
use sqlx::PgPool;
use std::time::Duration;

pub async fn start_scheduler(pool: PgPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(1800));
    loop {
        interval.tick().await;
        println!("Running scheduled import...");

        let nodes = api_client::fetch_nodes().await;
        node_repository::save_nodes(&pool, nodes).await;

        println!("Scheduled import completed");
    }
}
