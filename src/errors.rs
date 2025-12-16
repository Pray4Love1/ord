use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::fmt;

/// Unified error type for BRC-20 v2 operations
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
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
    Soulbound {
        address: String,
    },

    #[error("vesting locked until block {unlock_block} for address {address}")]
    VestingLocked {
        address: String,
        unlock_block: u64,
    },

    #[error("amount exceeds max per transaction: max={max}, attempted={attempted}")]
    MaxTransferExceeded {
        max: u64,
        attempted: u64,
        address: String,
    },

    #[error("invalid zk proof: {reason}")]
    InvalidProof {
        reason: Option<String>,
    },

    #[error("relay error: {source}")]
    Relay {
        source: String,
        chain: Option<String>,
    },

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl Brc20Error {
    /// Helper to create a new `Unexpected` error
    pub fn unexpected<S: Into<String>>(msg: S) -> Self {
        Brc20Error::Unexpected(msg.into())
    }

    /// Returns a user-friendly message without debug info
    pub fn friendly(&self) -> String {
        match self {
            Brc20Error::IdentityFailed { address, .. } => {
                format!("Identity verification failed for {}", address)
            }
            Brc20Error::InsufficientBalance { available, required, address } => {
                format!("Insufficient balance for {}: have {}, need {}", address, available, required)
            }
            Brc20Error::Soulbound { address } => {
                format!("Token is soulbound and cannot be transferred by {}", address)
            }
            Brc20Error::VestingLocked { address, unlock_block } => {
                format!("Tokens for {} are locked until block {}", address, unlock_block)
            }
            Brc20Error::MaxTransferExceeded { max, attempted, address } => {
                format!("Transfer of {} exceeds max allowed {} for {}", attempted, max, address)
            }
            Brc20Error::InvalidProof { reason } => {
                format!("Invalid zk proof: {}", reason.clone().unwrap_or("unknown".into()))
            }
            Brc20Error::Relay { source, chain } => {
                match chain {
                    Some(c) => format!("Relay error on {}: {}", c, source),
                    None => format!("Relay error: {}", source),
                }
            }
            Brc20Error::Unexpected(msg) => format!("Unexpected error: {}", msg),
        }
    }
}
