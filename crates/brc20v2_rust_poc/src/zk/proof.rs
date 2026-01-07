use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct ZkTransferProof {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub prev_state_hash: String,
    pub proof_hash: String,
}

impl ZkTransferProof {
    pub fn generate(from: &str, to: &str, amount: u64, prev: &str) -> Self {
        let payload = format!("{from}|{to}|{amount}|{prev}");
        let proof_hash = hex::encode(Sha256::digest(payload.as_bytes()));

        Self {
            from: from.into(),
            to: to.into(),
            amount,
            prev_state_hash: prev.into(),
            proof_hash,
        }
    }
}
