use {
  crate::errors::Brc20Error,
  chrono::Utc,
  serde::{Deserialize, Serialize},
  sha2::{Digest, Sha256},
};

/// Full-featured zk-proof structure.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ZkProof {
  pub from: String,
  pub to: String,
  pub amount: u64,
  pub prev_state_hash: String,
  pub timestamp: i64,
  pub identity_verified: bool,
  pub proof_hash: String,
  pub version: String,
}

impl ZkProof {
  /// Generate a new zk proof (placeholder for Halo2 / zk-SNARK integration).
  pub fn new(
    from: &str,
    to: &str,
    amount: u64,
    prev_state_hash: &str,
    identity_verified: bool,
  ) -> Result<Self, Brc20Error> {
    if !identity_verified {
      return Err(Brc20Error::IdentityFailed);
    }

    let proof_hash = Self::hash_payload(from, to, amount, prev_state_hash);

    Ok(Self {
      from: from.to_string(),
      to: to.to_string(),
      amount,
      prev_state_hash: prev_state_hash.to_string(),
      timestamp: Utc::now().timestamp(),
      identity_verified,
      proof_hash,
      version: "v2".to_string(),
    })
  }

  /// Verify the proof (currently hash-based placeholder).
  pub fn verify(&self) -> Result<bool, Brc20Error> {
    if !self.identity_verified {
      return Err(Brc20Error::IdentityFailed);
    }

    let expected_hash = Self::hash_payload(&self.from, &self.to, self.amount, &self.prev_state_hash);

    if expected_hash != self.proof_hash {
      return Err(Brc20Error::InvalidProof);
    }

    Ok(true)
  }

  /// Export canonical JSON for inscription.
  pub fn export_canonical(&self) -> Result<String, Brc20Error> {
    serde_json::to_string(self).map_err(|error| Brc20Error::Relay(error.to_string()))
  }

  fn hash_payload(from: &str, to: &str, amount: u64, prev_state_hash: &str) -> String {
    let data = format!("{}|{}|{}|{}", from, to, amount, prev_state_hash);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_zkproof_generation_and_verify() {
    let proof = ZkProof::new("bc1qalice...", "bc1qbob...", 100, "0000abcd", true).unwrap();
    assert!(proof.verify().unwrap());

    let json = proof.export_canonical().unwrap();
    assert!(json.contains("bc1qalice"));
    assert!(json.contains("bc1qbob"));
  }

  #[test]
  fn test_zkproof_identity_fail() {
    let result = ZkProof::new("a", "b", 10, "prevhash", false);
    assert!(matches!(result, Err(Brc20Error::IdentityFailed)));
  }

  #[test]
  fn test_zkproof_invalid_hash() {
    let mut proof = ZkProof::new("a", "b", 10, "prevhash", true).unwrap();
    proof.proof_hash = "wronghash".to_string();
    assert!(matches!(proof.verify(), Err(Brc20Error::InvalidProof)));
  }
}
