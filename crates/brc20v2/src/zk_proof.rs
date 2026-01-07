use sha2::{Digest, Sha256};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofRequest {
    pub statement: String,
    pub witness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    pub proof: String,
}

impl ZkProof {
    /// High-level ZK-style proof generator
    pub fn generate(request: &ZkProofRequest) -> Result<Self> {
        if request.statement.trim().is_empty() {
            return Err(anyhow!("statement cannot be empty"));
        }
        let proof = format!("zkp:{}:{}", request.statement, request.witness);
        Ok(Self { proof })
    }
}

/// Lightweight mock proof generator for internal BRC20 state relay
pub fn generate_zk_proof(
    from: &str,
    to: &str,
    amount: u64,
    prev_state_hash: &str,
    identity_verified: bool,
) -> String {
    if !identity_verified {
        panic!("Identity verification failed");
    }

    let data = format!("{}|{}|{}|{}", from, to, amount, prev_state_hash);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}
