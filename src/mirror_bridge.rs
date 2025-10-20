use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::living_inscription::LivingInscription;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorProof {
  pub inscription: LivingInscription,
  pub target_chain: String,
  pub proof_hash: String,
  pub issued_at: DateTime<Utc>,
}

pub fn build_proof(inscription: &LivingInscription, target_chain: &str) -> MirrorProof {
  let mut hasher = Sha3_256::new();
  hasher.update(inscription.core.creator.as_bytes());
  hasher.update(inscription.signature.as_bytes());
  hasher.update(target_chain.as_bytes());
  let proof_hash = hasher.finalize();

  MirrorProof {
    inscription: inscription.clone(),
    target_chain: target_chain.to_owned(),
    proof_hash: hex::encode(proof_hash),
    issued_at: Utc::now(),
  }
}

pub async fn post_proof(
  http: &Client,
  proof: &MirrorProof,
  mirror_rpc: &str,
  verifier: &str,
) -> Result<()> {
  let payload = serde_json::json!({
      "verifier": verifier,
      "target_chain": proof.target_chain,
      "proof_hash": proof.proof_hash,
      "issued_at": proof.issued_at,
      "inscription": proof.inscription,
  });

  let response = http.post(mirror_rpc).json(&payload).send().await?;
  println!(
    "Submitted proof {} to verifier {} (status: {})",
    proof.proof_hash,
    verifier,
    response.status()
  );

  Ok(())
}
