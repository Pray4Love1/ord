use sha2::{Digest, Sha256};

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
