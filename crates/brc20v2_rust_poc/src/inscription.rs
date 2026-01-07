use serde::Serialize;

#[derive(Serialize)]
pub struct Inscription {
    pub protocol: &'static str,
    pub action: String,
    pub token_id: String,
    pub state_hash: String,
    pub merkle_root: String,
    pub proof: Option<serde_json::Value>,
    pub timestamp: i64,
}
