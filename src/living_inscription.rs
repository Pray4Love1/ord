//! Living Inscription v0.1 (Rust 2024)
//! Specification and reference model for dynamic, evolvable Ordinals.
//!
//! Compile with: cargo +nightly build -Zunstable-options --edition 2024
//! Requires: serde = { version = "1", features = ["derive"] }, blake3, chrono

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Immutable root data committed in the Bitcoin inscription.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InscriptionCore {
  pub version: u8,
  pub parent_hash: Option<String>, // prior inscription reference
  pub creator: String,             // address or key fingerprint
  pub timestamp: DateTime<Utc>,    // ISO-8601 UTC
  pub content_uri: String,         // ipfs://, ar://, or inline data
  pub metadata: serde_json::Value, // arbitrary JSON metadata
}

/// Mutable “living” overlay verified off-chain but cryptographically committed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InscriptionState {
  pub block_height: u64,
  pub external_entropy: Option<String>, // pulse / env / noise hash
  pub mood: Option<String>,             // optional human context
  pub mirror_hash: Option<String>,      // cross-chain reflection
}

/// The complete living inscription object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LivingInscription {
  pub core: InscriptionCore,
  pub state: InscriptionState,
  pub signature: String, // signed by creator over blake3(core||state)
}

impl LivingInscription {
  /// Deterministically hash the inscription’s data for verification.
  pub fn commitment(&self) -> String {
    let payload = serde_json::to_vec(self).expect("serialize");
    blake3::hash(&payload).to_hex().to_string()
  }

  /// Example reactor: evolve automatically when a new block height is reached.
  pub fn evolve(&mut self, new_height: u64) {
    if new_height > self.state.block_height {
      self.state.block_height = new_height;
      self.state.mood = Some(format!("Awakened at block {}", new_height));
    }
  }
}

/// Target chain configuration for bridge proofs.
#[derive(Debug, Clone)]
pub struct MirrorBridgeConfig {
  pub chain_name: String,
  pub chain_id: u64,
  pub contract_address: String,
}

/// Compact representation of a living inscription commitment for EVM calldata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MirrorProof {
  pub chain_name: String,
  pub chain_id: u64,
  pub contract_address: String,
  pub commitment: String,
  pub signature: String,
  pub block_height: u64,
  pub timestamp: DateTime<Utc>,
  pub metadata_root: Option<String>,
}

impl MirrorProof {
  /// Encode the proof into a deterministic byte payload suitable for calldata.
  pub fn encode(&self) -> Vec<u8> {
    fn encode_string(dst: &mut Vec<u8>, value: &str) {
      let len = value.len() as u32;
      dst.extend_from_slice(&len.to_be_bytes());
      dst.extend_from_slice(value.as_bytes());
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&self.chain_id.to_be_bytes());
    encode_string(&mut bytes, &self.chain_name);
    encode_string(&mut bytes, &self.contract_address);
    encode_string(&mut bytes, &self.commitment);
    encode_string(&mut bytes, &self.signature);
    bytes.extend_from_slice(&self.block_height.to_be_bytes());
    bytes.extend_from_slice(&self.timestamp.timestamp_millis().to_be_bytes());

    match &self.metadata_root {
      Some(root) => {
        bytes.push(1);
        encode_string(&mut bytes, root);
      }
      None => bytes.push(0),
    }

    bytes
  }
}

/// Error type returned by the mirror bridge.
#[derive(Debug)]
pub enum MirrorBridgeError {
  MetadataSerialization(serde_json::Error),
  Transmission(String),
}

impl core::fmt::Display for MirrorBridgeError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::MetadataSerialization(err) => write!(f, "metadata serialization failed: {err}"),
      Self::Transmission(msg) => write!(f, "transmission error: {msg}"),
    }
  }
}

impl std::error::Error for MirrorBridgeError {}

impl From<serde_json::Error> for MirrorBridgeError {
  fn from(value: serde_json::Error) -> Self {
    Self::MetadataSerialization(value)
  }
}

/// Trait abstraction for dispatching proofs to external transports.
pub trait ProofTransmitter {
  fn transmit(&self, proof: &MirrorProof) -> Result<(), MirrorBridgeError>;
}

/// Stateless bridge that derives commitment proofs and forwards them to EVM chains.
pub struct MirrorBridge<T: ProofTransmitter> {
  config: MirrorBridgeConfig,
  transmitter: T,
}

impl<T: ProofTransmitter> MirrorBridge<T> {
  pub fn new(config: MirrorBridgeConfig, transmitter: T) -> Self {
    Self {
      config,
      transmitter,
    }
  }

  /// Build a compact proof for the provided living inscription.
  pub fn build_proof(
    &self,
    inscription: &LivingInscription,
  ) -> Result<MirrorProof, MirrorBridgeError> {
    let metadata_root = if inscription.core.metadata.is_null() {
      None
    } else {
      let metadata_bytes = serde_json::to_vec(&inscription.core.metadata)?;
      Some(blake3::hash(&metadata_bytes).to_hex().to_string())
    };

    Ok(MirrorProof {
      chain_name: self.config.chain_name.clone(),
      chain_id: self.config.chain_id,
      contract_address: self.config.contract_address.clone(),
      commitment: inscription.commitment(),
      signature: inscription.signature.clone(),
      block_height: inscription.state.block_height,
      timestamp: inscription.core.timestamp,
      metadata_root,
    })
  }

  /// Produce and dispatch the proof to the configured transmitter.
  pub fn mirror(&self, inscription: &LivingInscription) -> Result<(), MirrorBridgeError> {
    let proof = self.build_proof(inscription)?;
    self.transmitter.transmit(&proof)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_inscription() -> LivingInscription {
    let core = InscriptionCore {
      version: 1,
      parent_hash: None,
      creator: "bc1qexample".into(),
      timestamp: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
      content_uri: "ipfs://QmExample".into(),
      metadata: serde_json::json!({"type":"art"}),
    };
    let state = InscriptionState {
      block_height: 830_000,
      external_entropy: Some("noisehash123".into()),
      mood: Some("serene".into()),
      mirror_hash: None,
    };
    LivingInscription {
      core,
      state,
      signature: "0xdeadbeefsig".into(),
    }
  }

  #[test]
  fn test_commitment_length() {
    let living = sample_inscription();
    let commitment = living.commitment();
    assert_eq!(commitment.len(), 64);
  }

  use std::sync::{Arc, Mutex};

  #[derive(Clone)]
  struct MockTransmitter {
    proofs: Arc<Mutex<Vec<MirrorProof>>>,
  }

  impl MockTransmitter {
    fn new(proofs: Arc<Mutex<Vec<MirrorProof>>>) -> Self {
      Self { proofs }
    }
  }

  impl ProofTransmitter for MockTransmitter {
    fn transmit(&self, proof: &MirrorProof) -> Result<(), MirrorBridgeError> {
      self.proofs.lock().unwrap().push(proof.clone());
      Ok(())
    }
  }

  #[test]
  fn test_bridge_builds_and_transmits_proof() {
    let inscription = sample_inscription();
    let proofs = Arc::new(Mutex::new(Vec::new()));
    let transmitter = MockTransmitter::new(Arc::clone(&proofs));
    let bridge = MirrorBridge::new(
      MirrorBridgeConfig {
        chain_name: "ethereum".into(),
        chain_id: 1,
        contract_address: "0xproofcontract".into(),
      },
      transmitter,
    );

    bridge.mirror(&inscription).unwrap();

    let stored = proofs.lock().unwrap();
    assert_eq!(stored.len(), 1);
    let proof = &stored[0];
    assert_eq!(proof.commitment, inscription.commitment());
    assert_eq!(proof.block_height, inscription.state.block_height);
    assert!(proof.metadata_root.is_some());
  }

  #[test]
  fn proof_encode_is_deterministic() {
    let inscription = sample_inscription();
    let bridge = MirrorBridge::new(
      MirrorBridgeConfig {
        chain_name: "sei".into(),
        chain_id: 1329,
        contract_address: "0xmirror".into(),
      },
      MockTransmitter::new(Arc::new(Mutex::new(Vec::new()))),
    );
    let proof = bridge.build_proof(&inscription).unwrap();
    let encoded_once = proof.encode();
    let encoded_twice = proof.encode();
    assert_eq!(encoded_once, encoded_twice);
  }
}
