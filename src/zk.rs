use serde::{Deserialize, Serialize};

use crate::errors::Brc20Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
  pub statement: String,
  pub proof: String,
}

impl ZkProof {
  pub fn generate(from: &str, to: &str, amount: u64, prev_state_hash: &str) -> Result<Self, Brc20Error> {
    let statement = format!(
      "transfer:{}:{}:{}:{}",
      from, to, amount, prev_state_hash
    );
    let proof = format!("proof:{}", statement.len());
    Ok(Self { statement, proof })
  }
}
