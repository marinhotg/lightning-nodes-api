use crate::models::node::{ApiNode, NodeResponse};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use axum::http::StatusCode;

pub async fn save_nodes(pool: &PgPool, nodes: Vec<ApiNode>) -> Result<(), StatusCode> {
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
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let deleted = sqlx::query("DELETE FROM nodes WHERE updated_at < $1")
        .bind(operation_start)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Deleted {} outdated nodes", deleted.rows_affected());
    Ok(())
}

pub async fn get_all_nodes(pool: &PgPool) -> Result<Vec<NodeResponse>, StatusCode> {
    let rows = sqlx::query(
        "SELECT public_key, alias, capacity_btc::FLOAT8 as capacity_btc_float, first_seen_formatted FROM nodes ORDER BY capacity_sats DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut nodes = Vec::new();
    for row in rows {
        let public_key: String = row.get("public_key");
        let alias: String = row.get("alias");
        let capacity_btc: f64 = row.get("capacity_btc_float");
        let first_seen_formatted: DateTime<Utc> = row.get("first_seen_formatted");

        nodes.push(NodeResponse {
            public_key,
            alias,
            capacity: format!("{:.8}", capacity_btc),
            first_seen: first_seen_formatted
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
        });
    }

    Ok(nodes)
}
