use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

const DOMAIN: &str = "brc20v2.zk.transfer";

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}
