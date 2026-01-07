use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Brc20Error {
  Soulbound,
  VestingLocked(u64),
  MaxTransferExceeded,
  IdentityFailed,
  InsufficientBalance,
  ZkProofFailed(String),
  IdentityFailed,
  InvalidProof,
  Relay(String),
}

impl fmt::Display for Brc20Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Brc20Error::Soulbound => write!(f, "token is soulbound"),
      Brc20Error::VestingLocked(block) => write!(f, "vesting locked until block {}", block),
      Brc20Error::MaxTransferExceeded => write!(f, "max transfer exceeded"),
      Brc20Error::IdentityFailed => write!(f, "identity verification failed"),
      Brc20Error::InsufficientBalance => write!(f, "insufficient balance"),
      Brc20Error::ZkProofFailed(reason) => write!(f, "zk proof failed: {}", reason),
      Self::IdentityFailed => write!(f, "identity verification failed"),
      Self::InvalidProof => write!(f, "invalid proof"),
      Self::Relay(message) => write!(f, "{}", message),
    }
  }
}

impl std::error::Error for Brc20Error {}
