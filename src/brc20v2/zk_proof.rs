use bitcoin::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZkProof {
  pub statement_hash: [u8; 32],
  pub proof_hash: [u8; 32],
}

impl ZkProof {
  pub fn new(statement_bytes: &[u8], witness_bytes: &[u8]) -> Self {
    let statement_hash = sha256::Hash::hash(statement_bytes).to_byte_array();
    let proof_hash = sha256::Hash::hash(&[statement_bytes, witness_bytes].concat()).to_byte_array();
    Self {
      statement_hash,
      proof_hash,
    }
  }

  pub fn verify(&self, statement_bytes: &[u8]) -> bool {
    let expected_statement_hash = sha256::Hash::hash(statement_bytes).to_byte_array();
    self.statement_hash == expected_statement_hash
  }
}
