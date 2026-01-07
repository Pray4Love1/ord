use serde::{Deserialize, Serialize};

/// Placeholder for zero-knowledge proofs used in inscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
  pub payload: serde_json::Value,
}
