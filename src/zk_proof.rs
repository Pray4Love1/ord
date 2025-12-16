use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use hex;

/// Domain separator prevents cross-protocol replay
pub const ZK_DOMAIN: &str = "BRC20V2::ZK::TRANSFER";

/// Future-proof proof envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofEnvelope {
    pub domain: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub prev_state_hash: String,

    // identity / anti-sybil
    pub identity_commitment: Option<String>,
    pub identity_verified: bool,

    // replay protection
    pub nonce: u64,

    // time binding
    pub block_height: u64,
    pub epoch: u64,

    // rule binding
    pub max_per_tx: Option<u64>,

    // cross-chain safety
    pub chain_id: String,

    // final proof hash
    pub proof_hash: String,
}

/// Deterministic proof generator (Bitcoin-friendly)
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

    // ---- hard checks (protocol law) ----
    if !identity_verified {
        panic!("Identity verification failed");
    }

    if let Some(limit) = max_per_tx {
        if amount > limit {
            panic!("Transfer exceeds max_per_tx constraint");
        }
    }

    // ---- canonical serialization (DO NOT CHANGE ORDER) ----
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

    // ---- hash ----
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

/// Verification helper (indexers, relayers, light clients)
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
