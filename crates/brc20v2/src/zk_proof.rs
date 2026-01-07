use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofEnvelope {
    pub proof_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub prev_state_hash: String,
    pub identity_verified: bool,
    pub identity_commitment: Option<String>,
    pub nonce: u64,
    pub block_height: u64,
    pub epoch: u64,
    pub max_per_tx: Option<u64>,
    pub chain_id: String,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn generate_zk_proof(
    from: &str,
    to: &str,
    amount: u64,
    prev_state_hash: &str,
    identity_verified: bool,
    identity_commitment: Option<&str>,
    nonce: u64,
    block_height: u64,
    epoch: u64,
    max_per_tx: Option<u64>,
    chain_id: &str,
) -> ZkProofEnvelope {
    let timestamp = Utc::now().timestamp() as u64;
    let mut hasher = Sha256::new();
    hasher.update(from.as_bytes());
    hasher.update(to.as_bytes());
    hasher.update(amount.to_le_bytes());
    hasher.update(prev_state_hash.as_bytes());
    hasher.update(&[identity_verified as u8]);
    if let Some(commitment) = identity_commitment {
        hasher.update(commitment.as_bytes());
    }
    hasher.update(nonce.to_le_bytes());
    hasher.update(block_height.to_le_bytes());
    hasher.update(epoch.to_le_bytes());
    if let Some(max) = max_per_tx {
        hasher.update(max.to_le_bytes());
    }
    hasher.update(chain_id.as_bytes());
    hasher.update(timestamp.to_le_bytes());

    ZkProofEnvelope {
        proof_id: hex::encode(hasher.finalize()),
        from: from.to_string(),
        to: to.to_string(),
        amount,
        prev_state_hash: prev_state_hash.to_string(),
        identity_verified,
        identity_commitment: identity_commitment.map(str::to_string),
        nonce,
        block_height,
        epoch,
        max_per_tx,
        chain_id: chain_id.to_string(),
        timestamp,
    }
}
