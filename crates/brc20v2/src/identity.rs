use sha2::{Digest, Sha256};

pub struct IdentityProof {
    pub subject: String,
    pub commitment: String,
}

pub fn verify_identity(subject: &str, signature: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(subject.as_bytes());
    let expected = hex::encode(hasher.finalize());
    expected == signature
}

pub fn generate_identity_commitment(subject: &str) -> IdentityProof {
    let mut hasher = Sha256::new();
    hasher.update(subject.as_bytes());
    IdentityProof {
        subject: subject.to_string(),
        commitment: hex::encode(hasher.finalize()),
    }
}
