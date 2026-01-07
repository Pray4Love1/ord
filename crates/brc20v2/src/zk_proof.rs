use sha2::{Digest, Sha256};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Domain separator prevents cross-protocol replay.
pub const ZK_DOMAIN: &str = "BRC20V2::ZK::TRANSFER";

/// Simple ZK-style request/response format (can be extended with real circuits).
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
    pub fn generate(request: &ZkProofRequest) -> Result<Self> {
        if request.statement.trim().is_empty() {
            return Err(anyhow!("statement cannot be empty"));
        }
        let proof = format!("zkp:{}:{}", request.statement, request.witness);
        Ok(Self { proof })
    }
}

/// Full envelope for deterministic, verifiable BRC20v2 proof relay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofEnvelope {
    pub domain: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub prev_state_hash: String,

    // Identity / anti-sybil layer
    pub identity_commitment: Option<String>,
    pub identity_verified: bool,

    // Replay protection
    pub nonce: u64,

    // Time binding
    pub block_height: u64,
    pub epoch: u64,

    // Policy rule
    pub max_per_tx: Option<u64>,

    // Cross-chain integrity
    pub chain_id: String,

    // Final hash (proof commitment)
    pub proof_hash: String,
}

/// Main proof generator for state-layer BRC20v2 transfer
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
    if !identity_verified {
        panic!("Identity verification failed");
    }

    if let Some(limit) = max_per_tx {
        if amount > limit {
            panic!("Transfer exceeds max_per_tx constraint");
        }
    }

    // Canonical serialization (DO NOT change field order!)
    let canonical = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ZK_DOMAIN,
        from,
        to,
        amount,
        prev_state_hash,
        identity_commitment.unwrap_or(""),
        nonce,
        block_height,
        epoch,
        chain_id
    );

    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let proof_hash = hex::encode(hasher.finalize());

    ZkProofEnvelope {
        domain: ZK_DOMAIN.to_string(),
        from: from.to_string(),
        to: to.to_string(),
        amount,
        prev_state_hash: prev_state_hash.to_string(),
        identity_commitment: identity_commitment.map(|s| s.to_string()),
        identity_verified,
        nonce,
        block_height,
        epoch,
        max_per_tx,
        chain_id: chain_id.to_string(),
        proof_hash,
    }
}

/// Lightweight verifier — can be used by indexers or bridges
pub fn verify_zk_proof(proof: &ZkProofEnvelope) -> bool {
    let canonical = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        proof.domain,
        proof.from,
        proof.to,
        proof.amount,
        proof.prev_state_hash,
        proof.identity_commitment.clone().unwrap_or_default(),
        proof.nonce,
        proof.block_height,
        proof.epoch,
        proof.chain_id
    );

    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let expected = hex::encode(hasher.finalize());

    expected == proof.proof_hash
}
