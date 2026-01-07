use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProof {
  pub subject: String,
  pub commitment: String,
}

pub fn verify_identity(subject: &str, commitment: &str) -> bool {
  !subject.is_empty() && !commitment.is_empty()
}
