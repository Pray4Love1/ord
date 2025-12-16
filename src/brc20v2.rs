use std::collections::HashMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use crate::zk_proof::generate_zk_proof;
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub soulbound: bool,
    pub transfer_rules: HashMap<String,u64>,
    pub vesting: HashMap<String,u64>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BRC20v2 {
    pub token_id: String,
    pub balances: HashMap<String,u64>,
    pub metadata: Metadata,
    pub prev_state_hash: String,
    pub merkle_root: String
}

#[derive(Serialize)]
pub struct Inscription {
    pub inscription_type: String,
    pub token_id: String,
    pub action: String,
    pub state_hash: String,
    pub merkle_root: String,
    pub metadata: Metadata,
    pub proof: Option<String>,
    pub timestamp: i64
}

impl BRC20v2 {
    pub fn new(token_id: &str) -> Self {
        Self {
            token_id: token_id.to_string(),
            balances: HashMap::new(),
            metadata: Metadata{
                name: "".to_string(),
                symbol: "".to_string(),
                decimals: 0,
                soulbound: false,
                transfer_rules: HashMap::new(),
                vesting: HashMap::new()
            },
            prev_state_hash: "0".repeat(64),
            merkle_root: "".to_string()
        }
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn mint(&mut self, addr: &str, amount: u64) {
        *self.balances.entry(addr.to_string()).or_insert(0) += amount;
        self.update_state();
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64, current_block: u64, identity_verified: bool) -> String {
        if self.metadata.soulbound {
            panic!("Soulbound token cannot be transferred");
        }
        if let Some(&unlock_block) = self.metadata.vesting.get(from) {
            if current_block < unlock_block {
                panic!("Tokens are locked until block {}", unlock_block);
            }
        }
        if let Some(&max_per_tx) = self.metadata.transfer_rules.get("max_per_tx") {
            if amount > max_per_tx {
                panic!("Exceeds max per transaction");
            }
        }
        let from_balance = self.balances.get_mut(from).expect("Sender balance not found");
        if *from_balance < amount {
            panic!("Insufficient balance");
        }
        *from_balance -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        let proof = generate_zk_proof(from, to, amount, &self.prev_state_hash, identity_verified);
        self.update_state();
        proof
    }

    pub fn update_state(&mut self) {
        let mut leaves: Vec<String> = self.balances.iter().map(|(k,v)| format!("{}:{}",k,v)).collect();
        let mut nodes: Vec<String> = leaves.iter().map(|x| {
            let mut hasher = Sha256::new();
            hasher.update(x.as_bytes());
            hex::encode(hasher.finalize())
        }).collect();

        while nodes.len() > 1 {
            let mut temp: Vec<String> = vec![];
            let mut i = 0;
            while i < nodes.len() {
                let combined = if i+1 < nodes.len() {
                    format!("{}{}", nodes[i], nodes[i+1])
                } else {
                    nodes[i].clone()
                };
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                temp.push(hex::encode(hasher.finalize()));
                i += 2;
            }
            nodes = temp;
        }
        self.merkle_root = if !nodes.is_empty() { nodes[0].clone() } else { "".to_string() };
        let state = serde_json::json!({
            "token_id": self.token_id,
            "balances": self.balances,
            "metadata": self.metadata,
            "merkle_root": self.merkle_root,
            "timestamp": Utc::now().timestamp()
        });
        let mut hasher = Sha256::new();
        hasher.update(state.to_string().as_bytes());
        self.prev_state_hash = hex::encode(hasher.finalize());
    }

    pub fn generate_inscription(&self, action: &str, proof: Option<String>) -> Inscription {
        Inscription{
            inscription_type: "brc20v2".to_string(),
            token_id: self.token_id.clone(),
            action: action.to_string(),
            state_hash: self.prev_state_hash.clone(),
            merkle_root: self.merkle_root.clone(),
            metadata: self.metadata.clone(),
            proof,
            timestamp: Utc::now().timestamp()
        }
    }
}
