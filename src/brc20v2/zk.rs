use serde::{Deserialize, Serialize};

use super::zk_proof::ZkProof;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZkStatement {
  pub operation: String,
  pub token: String,
  pub from: Option<[u8; 32]>,
  pub to: Option<[u8; 32]>,
  pub amount: u128,
  pub merkle_root: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZkWitness {
  pub from_balance: u128,
  pub to_balance: u128,
}

pub trait ZkProofGenerator {
  fn generate(&self, statement: &ZkStatement, witness: &ZkWitness) -> ZkProof;
  fn verify(&self, statement: &ZkStatement, proof: &ZkProof) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct PlaceholderZk;

impl ZkProofGenerator for PlaceholderZk {
  fn generate(&self, statement: &ZkStatement, witness: &ZkWitness) -> ZkProof {
    let statement_bytes = serde_json::to_vec(statement).unwrap_or_default();
    let witness_bytes = serde_json::to_vec(witness).unwrap_or_default();
    ZkProof::new(&statement_bytes, &witness_bytes)
  }

  fn verify(&self, statement: &ZkStatement, proof: &ZkProof) -> bool {
    let statement_bytes = serde_json::to_vec(statement).unwrap_or_default();
    proof.verify(&statement_bytes)
  }
}
