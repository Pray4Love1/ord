use sha2::{Digest, Sha256};

use crate::errors::Brc20Error;

#[derive(Debug, Clone)]
pub struct ZkProof {
    pub commitment: String,
}

pub fn generate_proof(
    from: &str,
    to: &str,
    amount: u64,
    prev_hash: &str,
    identity_verified: bool,
) -> Result<ZkProof, Brc20Error> {
    if !identity_verified {
        return Err(Brc20Error::IdentityFailed);
    }

    let mut h = Sha256::new();
    h.update(format!("{}|{}|{}|{}", from, to, amount, prev_hash));

    Ok(ZkProof {
        commitment: hex::encode(h.finalize()),
    })
}

pub fn verify_proof(proof: &ZkProof) -> bool {
    !proof.commitment.is_empty()
}
