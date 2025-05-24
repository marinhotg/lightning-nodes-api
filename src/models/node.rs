use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ApiNode {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: String,
    pub capacity: i64,
    #[serde(rename = "firstSeen")]
    pub first_seen: i64,
}
