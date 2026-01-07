use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofRequest {
  pub statement: String,
  pub witness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
  pub proof: String,
}

impl ZkProof {
  pub fn generate(request: &ZkProofRequest) -> Result<Self> {
    if request.statement.trim().is_empty() {
      return Err(anyhow!("statement cannot be empty"));
    }

    let proof = format!("proof:{}:{}", request.statement, request.witness);
    Ok(Self { proof })
  }
}
