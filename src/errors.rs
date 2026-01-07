use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Brc20Error {
  IdentityFailed,
  InvalidProof,
  Relay(String),
}

impl fmt::Display for Brc20Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::IdentityFailed => write!(f, "identity verification failed"),
      Self::InvalidProof => write!(f, "invalid proof"),
      Self::Relay(message) => write!(f, "{}", message),
    }
  }
}

impl std::error::Error for Brc20Error {}
