use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

const DOMAIN: &str = "brc20v2.zk.transfer";

#[derive(Serialize, Deserialize, Debug, Clone)]
use sha2::{Digest, Sha256};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// Domain separator to avoid cross-protocol replay
pub const ZK_DOMAIN: &str = "BRC20V2::ZK::TRANSFER";

/// Minimal request structure for mock or future zk-SNARK integration
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

/// Final envelope for cross-chain BRC20v2 verifiable proof relays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofEnvelope {
    pub domain: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub prev_state_hash: String,
    pub nonce: u64,
    pub timestamp: u64,
    pub chain_id: u32,
    pub identity_verified: bool,
    pub proof_hash: String,
}

impl ZkProofEnvelope {
    pub fn generate(
        from: &str,
        to: &str,
        amount: u64,
        prev_state_hash: &str,
        nonce: u64,
        chain_id: u32,
        identity_verified: bool,
    ) -> Self {
        if !identity_verified {
            panic!("identity verification failed");
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_secs();

        let payload = format!(
            "{DOMAIN}|{from}|{to}|{amount}|{prev_state_hash}|{nonce}|{chain_id}"
        );

        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let proof_hash = hex::encode(hasher.finalize());

        Self {
            domain: DOMAIN.into(),
            from: from.into(),
            to: to.into(),
            amount,
            prev_state_hash: prev_state_hash.into(),
            nonce,
            timestamp,
            chain_id,
            identity_verified,
            proof_hash,
        }
    }

    // Identity/Anti-sybil Layer
    pub identity_verified: bool,
    pub identity_commitment: Option<String>,

    // Replay Protection
    pub nonce: u64,

    // Time Binding
    pub block_height: u64,
    pub epoch: u64,
    pub timestamp: u64,

    // Policy
    pub max_per_tx: Option<u64>,

    // Cross-chain integrity
    pub chain_id: String,

    // Final hash for signature/validation
    pub proof_hash: String,
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
    if !identity_verified {
        panic!("Identity verification failed");
    }

    if let Some(limit) = max_per_tx {
        if amount > limit {
            panic!("Transfer exceeds max_per_tx constraint");
        }
    }

    let timestamp = Utc::now().timestamp() as u64;

    // Canonical, deterministic proof payload
    let canonical = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ZK_DOMAIN,
        from,
        to,
        amount,
        prev_state_hash,
        identity_commitment.unwrap_or(""),
        nonce,
        block_height,
        epoch,
        chain_id,
        timestamp
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
        identity_verified,
        identity_commitment: identity_commitment.map(str::to_string),
        nonce,
        block_height,
        epoch,
        timestamp,
        max_per_tx,
        chain_id: chain_id.to_string(),
        proof_hash,
    }
}

/// Verify zk-proof envelope without external signature or SNARK
pub fn verify_zk_proof(proof: &ZkProofEnvelope) -> bool {
    let canonical = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        proof.domain,
        proof.from,
        proof.to,
        proof.amount,
        proof.prev_state_hash,
        proof.identity_commitment.clone().unwrap_or_default(),
        proof.nonce,
        proof.block_height,
        proof.epoch,
        proof.chain_id,
        proof.timestamp
    );

    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let expected = hex::encode(hasher.finalize());

    expected == proof.proof_hash
}
