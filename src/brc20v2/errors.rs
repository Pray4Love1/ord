use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Brc20Error {
  TokenAlreadyExists(String),
  TokenNotFound(String),
  InvalidOperation(String),
  InsufficientBalance {
    available: u128,
    required: u128,
  },
  MintLimitExceeded {
    limit: u128,
    requested: u128,
  },
  MaxSupplyExceeded {
    max_supply: u128,
    attempted_total: u128,
  },
  SoulboundTransferDenied(String),
  VestingScheduleInvalid(String),
  ProofVerificationFailed(String),
}

impl fmt::Display for Brc20Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::TokenAlreadyExists(ticker) => {
        write!(f, "token {ticker} already exists")
      }
      Self::TokenNotFound(ticker) => {
        write!(f, "token {ticker} not found")
      }
      Self::InvalidOperation(message) => f.write_str(message),
      Self::InsufficientBalance { available, required } => {
        write!(f, "insufficient balance: {available} available, {required} required")
      }
      Self::MintLimitExceeded { limit, requested } => {
        write!(f, "mint limit {limit} exceeded by request {requested}")
      }
      Self::MaxSupplyExceeded {
        max_supply,
        attempted_total,
      } => write!(
        f,
        "max supply {max_supply} exceeded by attempted total {attempted_total}"
      ),
      Self::SoulboundTransferDenied(ticker) => {
        write!(f, "token {ticker} is soulbound and cannot be transferred")
      }
      Self::VestingScheduleInvalid(message) => {
        write!(f, "invalid vesting schedule: {message}")
      }
      Self::ProofVerificationFailed(message) => {
        write!(f, "proof verification failed: {message}")
      }
    }
  }
}

impl Error for Brc20Error {}
