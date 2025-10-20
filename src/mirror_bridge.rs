//! Shared types for interacting with the mirror verifier contract.

use ethers::types::{Address, H256};
use serde::{Deserialize, Serialize};

/// Minimal representation of a mirror record returned from the verifier contract.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MirrorRecord {
  /// Address of the inscription's creator on the target chain.
  pub creator: Address,
  /// Content hash committed to when the mirror was created.
  pub content_hash: H256,
  /// Block height on which the mirror was finalized.
  pub block_height: u64,
  /// Millisecond timestamp recorded by the mirror bridge.
  pub timestamp_ms: u64,
}

impl From<(Address, [u8; 32], u64, u64)> for MirrorRecord {
  fn from(value: (Address, [u8; 32], u64, u64)) -> Self {
    Self {
      creator: value.0,
      content_hash: H256::from_slice(&value.1),
      block_height: value.2,
      timestamp_ms: value.3,
    }
  }
}
