use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ApiNode {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: String,
    pub capacity: i64,
    #[serde(rename = "firstSeen")]
    pub first_seen: i64,
}

#[derive(Serialize)]
pub struct NodeResponse {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
}
