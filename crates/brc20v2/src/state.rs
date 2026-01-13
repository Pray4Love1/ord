use std::collections::HashMap;

use crate::merkle::merkle_root;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct TokenState {
    pub balances: HashMap<String, u64>,
    pub prev_state_hash: String,
    pub merkle_root: String,
}

impl TokenState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            prev_state_hash: "0".repeat(64),
            merkle_root: String::new(),
        }
    }

    pub fn update(&mut self) {
        let leaves: Vec<String> = self
            .balances
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect();

        self.merkle_root = merkle_root(&leaves);

        let mut h = Sha256::new();
        h.update(format!("{:?}{}", self.balances, self.merkle_root));
        self.prev_state_hash = hex::encode(h.finalize());
    }
}
