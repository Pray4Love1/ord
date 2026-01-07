use std::collections::HashMap;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod cross_chain;
pub mod zk_proof;

pub use crate::cross_chain::{CrossChainRelay, RelayEnvelope, relay_to_ethereum};
pub use crate::zk_proof::{ZkProofEnvelope, generate_zk_proof, verify_zk_proof};

// Protocol Constants
pub const PROTOCOL: &str = "brc20v2";
pub const STATE_DOMAIN: &str = "BRC20V2::STATE";

// ---- Metadata & Rules ----
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRules {
    pub max_per_tx: Option<u64>,
    pub soulbound: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    pub unlock_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub rules: TokenRules,
}

// ---- State ----
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BRC20v2 {
    pub token_id: String,
    pub balances: HashMap<String, u64>,
    pub vesting: HashMap<String, VestingSchedule>,
    pub metadata: Metadata,
    pub merkle_root: String,
    pub prev_state_hash: String,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inscription {
    pub protocol: String,
    pub action: String,
    pub token_id: String,
    pub state_hash: String,
    pub merkle_root: String,
    pub proof: Option<ZkProofEnvelope>,
    pub metadata: Metadata,
    pub nonce: u64,
    pub timestamp: u64,
}

impl BRC20v2 {
    pub fn new(token_id: &str, metadata: Metadata) -> Self {
        Self {
            token_id: token_id.to_string(),
            balances: HashMap::new(),
            vesting: HashMap::new(),
            metadata,
            merkle_root: String::new(),
            prev_state_hash: "0".repeat(64),
            nonce: 0,
        }
    }

    pub fn mint(&mut self, to: &str, amount: u64) {
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        self.update_state();
    }

    pub fn add_vesting(&mut self, addr: &str, unlock_block: u64) {
        self.vesting.insert(addr.to_string(), VestingSchedule { unlock_block });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: u64,
        block_height: u64,
        epoch: u64,
        chain_id: &str,
        identity_verified: bool,
        identity_commitment: Option<&str>,
    ) -> ZkProofEnvelope {
        if self.metadata.rules.soulbound {
            panic!("Token is soulbound");
        }

        if let Some(v) = self.vesting.get(from) {
            if block_height < v.unlock_block {
                panic!("Tokens locked until block {}", v.unlock_block);
            }
        }

        if let Some(max) = self.metadata.rules.max_per_tx {
            if amount > max {
                panic!("Transfer exceeds max_per_tx");
            }
        }

        let sender = self.balances.get_mut(from).expect("Sender not found");
        if *sender < amount {
            panic!("Insufficient balance");
        }

        *sender -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;

        self.nonce += 1;

        let proof = generate_zk_proof(
            from,
            to,
            amount,
            &self.prev_state_hash,
            identity_verified,
            identity_commitment,
            self.nonce,
            block_height,
            epoch,
            self.metadata.rules.max_per_tx,
            chain_id,
        );

        self.update_state();
        proof
    }

    pub fn update_state(&mut self) {
        self.merkle_root = self.compute_merkle_root();

        let canonical = serde_json::json!({
            "domain": STATE_DOMAIN,
            "token_id": self.token_id,
            "balances": self.balances,
            "metadata": self.metadata,
            "merkle_root": self.merkle_root,
            "nonce": self.nonce,
            "timestamp": Utc::now().timestamp(),
        });

        let mut hasher = Sha256::new();
        hasher.update(canonical.to_string().as_bytes());
        self.prev_state_hash = hex::encode(hasher.finalize());
    }

    fn compute_merkle_root(&self) -> String {
        let mut leaves: Vec<String> = self
            .balances
            .iter()
            .map(|(a, b)| format!("{}:{}", a, b))
            .collect();
        leaves.sort();

        let mut nodes: Vec<String> = leaves
            .into_iter()
            .map(|l| {
                let mut h = Sha256::new();
                h.update(l.as_bytes());
                hex::encode(h.finalize())
            })
            .collect();

        while nodes.len() > 1 {
            let mut next = Vec::new();
            for pair in nodes.chunks(2) {
                let combined = if pair.len() == 2 {
                    format!("{}{}", pair[0], pair[1])
                } else {
                    pair[0].clone()
                };
                let mut h = Sha256::new();
                h.update(combined.as_bytes());
                next.push(hex::encode(h.finalize()));
            }
            nodes = next;
        }

        nodes.first().cloned().unwrap_or_default()
    }

    pub fn generate_inscription(
        &self,
        action: &str,
        proof: Option<ZkProofEnvelope>,
    ) -> Inscription {
        Inscription {
            protocol: PROTOCOL.to_string(),
            action: action.to_string(),
            token_id: self.token_id.clone(),
            state_hash: self.prev_state_hash.clone(),
            merkle_root: self.merkle_root.clone(),
            proof,
            metadata: self.metadata.clone(),
            nonce: self.nonce,
            timestamp: Utc::now().timestamp() as u64,
        }
    }
}
