use crate::models::node::ApiNode;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub async fn save_nodes(pool: &PgPool, nodes: Vec<ApiNode>) {
    let operation_start = Utc::now();

    println!("Starting sync operation at {}", operation_start);
    println!("Processing {} nodes", nodes.len());

    for node in nodes {
        let capacity_btc = node.capacity as f64 / 100_000_000.0;
        let first_seen_formatted = DateTime::from_timestamp(node.first_seen, 0).unwrap();

        sqlx::query(
            "INSERT INTO nodes (public_key, alias, capacity_sats, capacity_btc, first_seen_unix, first_seen_formatted, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, NOW())
             ON CONFLICT (public_key) DO UPDATE SET
             alias = $2, capacity_sats = $3, capacity_btc = $4, 
             first_seen_unix = $5, first_seen_formatted = $6, updated_at = NOW()"
        )
        .bind(&node.public_key)
        .bind(&node.alias)
        .bind(node.capacity)
        .bind(capacity_btc)
        .bind(node.first_seen)
        .bind(first_seen_formatted)
        .execute(pool)
        .await
        .unwrap();
    }

    let deleted = sqlx::query("DELETE FROM nodes WHERE updated_at < $1")
        .bind(operation_start)
        .execute(pool)
        .await
        .unwrap();

    println!("Deleted {} outdated nodes", deleted.rows_affected());
}
