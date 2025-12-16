use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeResult;
use crate::zk::ZkProof;

/// BRC-20 v2 inscription structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inscription<'a> {
    #[serde(default = "default_protocol")]
    pub protocol: &'static str,

    pub token: &'a str,
    pub action: &'a str,
    pub state_hash: &'a str,
    pub merkle_root: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<&'a ZkProof>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a serde_json::Value>,

    #[serde(default = "current_timestamp")]
    pub timestamp: u64,
}

fn default_protocol() -> &'static str {
    "brc20v2"
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl<'a> Inscription<'a> {
    /// Create a new inscription
    pub fn new(
        token: &'a str,
        action: &'a str,
        state_hash: &'a str,
        merkle_root: &'a str,
        proof: Option<&'a ZkProof>,
        metadata: Option<&'a serde_json::Value>,
    ) -> Self {
        Self {
            protocol: default_protocol(),
            token,
            action,
            state_hash,
            merkle_root,
            proof,
            metadata,
            timestamp: current_timestamp(),
        }
    }

    /// Serialize inscription to JSON string
    pub fn to_json(&self) -> SerdeResult<String> {
        serde_json::to_string_pretty(self)
    }

    /// Quick validation: ensures state_hash and merkle_root are not empty
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.state_hash.is_empty() {
            return Err("state_hash cannot be empty");
        }
        if self.merkle_root.is_empty() {
            return Err("merkle_root cannot be empty");
        }
        Ok(())
    }
}
