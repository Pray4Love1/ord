use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified error type for BRC-20 v2 operations.
#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum Brc20Error {
  #[error("identity verification failed for address {address}")]
  IdentityFailed {
    address: String,
    reason: Option<String>,
  },
  #[error("insufficient balance: available={available}, required={required}")]
  InsufficientBalance {
    available: u64,
    required: u64,
    address: String,
  },
  #[error("token is soulbound; transfer not allowed for address {address}")]
  Soulbound { address: String },
  #[error("vesting locked until block {unlock_block} for address {address}")]
  VestingLocked { address: String, unlock_block: u64 },
  #[error("amount exceeds max per transaction: max={max}, attempted={attempted}")]
  MaxTransferExceeded {
    max: u64,
    attempted: u64,
    address: String,
  },
  #[error("invalid zk proof: {reason}")]
  InvalidProof { reason: Option<String> },
  #[error("relay error: {source}")]
  Relay {
    source: String,
    chain: Option<String>,
  },
  #[error("unexpected error: {0}")]
  Unexpected(String),
}

impl Brc20Error {
  /// Helper to create a new `Unexpected` error.
  pub fn unexpected<S: Into<String>>(msg: S) -> Self {
    Self::Unexpected(msg.into())
  }

  /// Returns a user-friendly message without debug info.
  pub fn friendly(&self) -> String {
    match self {
      Self::IdentityFailed { address, .. } => {
        format!("Identity verification failed for {address}")
      }
      Self::InsufficientBalance {
        available,
        required,
        address,
      } => format!(
        "Insufficient balance for {address}: have {available}, need {required}"
      ),
      Self::Soulbound { address } => {
        format!("Token is soulbound and cannot be transferred by {address}")
      }
      Self::VestingLocked {
        address,
        unlock_block,
      } => format!("Tokens for {address} are locked until block {unlock_block}"),
      Self::MaxTransferExceeded {
        max,
        attempted,
        address,
      } => format!("Transfer of {attempted} exceeds max allowed {max} for {address}"),
      Self::InvalidProof { reason } => format!(
        "Invalid zk proof: {}",
        reason.clone().unwrap_or_else(|| "unknown".to_string())
      ),
      Self::Relay { source, chain } => match chain {
        Some(chain) => format!("Relay error on {chain}: {source}"),
        None => format!("Relay error: {source}"),
      },
      Self::Unexpected(msg) => format!("Unexpected error: {msg}"),
    }
  }
}
