use crate::repositories::node_repository;
use crate::services::api_client;
use sqlx::PgPool;
use std::time::Duration;

pub async fn start_scheduler(pool: PgPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(1800));

    loop {
        interval.tick().await;
        println!("Running scheduled import...");

        match api_client::fetch_nodes().await {
            Ok(nodes) => {
                if let Err(e) = node_repository::save_nodes(&pool, nodes).await {
                    println!("Failed to save nodes: {}", e);
                } else {
                    println!("Scheduled import completed");
                }
            }
            Err(e) => println!("Failed to fetch nodes: {}", e),
        }
    }
}
