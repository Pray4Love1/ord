use bitcoin::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentityCommitment {
  pub id: String,
  pub commitment: [u8; 32],
}

impl IdentityCommitment {
  pub fn new(id: impl Into<String>) -> Self {
    let id = id.into();
    let commitment = sha256::Hash::hash(id.as_bytes()).to_byte_array();
    Self { id, commitment }
  }

  pub fn with_commitment(id: impl Into<String>, commitment: [u8; 32]) -> Self {
    Self {
      id: id.into(),
      commitment,
    }
  }
}
