use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenState {
    pub token_id: String,
    pub balances: HashMap<String, u64>,
    pub metadata: serde_json::Value,
    pub prev_state_hash: String,
    pub merkle_root: String,
}

impl TokenState {
    pub fn new(token_id: &str) -> Self {
        Self {
            token_id: token_id.into(),
            balances: HashMap::new(),
            metadata: serde_json::json!({}),
            prev_state_hash: "0".repeat(64),
            merkle_root: String::new(),
        }
    }

    pub fn mint(&mut self, addr: &str, amount: u64) {
        *self.balances.entry(addr.into()).or_insert(0) += amount;
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> anyhow::Result<()> {
        let balance = self
            .balances
            .get_mut(from)
            .ok_or_else(|| anyhow::anyhow!("no balance"))?;
        if *balance < amount {
            anyhow::bail!("insufficient balance");
        }
        *balance -= amount;
        *self.balances.entry(to.into()).or_insert(0) += amount;
        Ok(())
    }

    pub fn compute_state_hash(&self) -> String {
        let json = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        hex::encode(hasher.finalize())
    }
}
