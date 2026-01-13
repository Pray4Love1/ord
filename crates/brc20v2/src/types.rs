use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenState {
    pub token_id: String,
    pub balances: HashMap<String, u64>,
    pub prev_state_hash: String,
    pub merkle_root: String,
}

impl TokenState {
    pub fn hash(&self) -> String {
        let json = serde_json::to_string(self).expect("token state serialization should succeed");
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) {
        let balance = self
            .balances
            .get_mut(from)
            .unwrap_or_else(|| panic!("missing sender balance for {from}"));
        if *balance < amount {
            panic!("insufficient balance");
        }
        *balance -= amount;
        *self.balances.entry(to.into()).or_insert(0) += amount;
        self.prev_state_hash = self.hash();
    }
}
