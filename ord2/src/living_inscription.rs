use anyhow::{Result, bail};
use chrono::{DateTime, TimeZone, Utc};
use ethers::types::{Address, H256};
use serde::{Deserialize, Serialize};

/// Representation of an inscription stored in the on-chain mirror verifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingInscription {
  pub creator: Address,
  pub commitment: H256,
  pub block_height: u64,
  pub timestamp_ms: u64,
}

impl LivingInscription {
  pub fn new(
    creator: Address,
    commitment: H256,
    block_height: u64,
    timestamp_ms: u64,
  ) -> Result<Self> {
    if commitment == H256::zero() {
      bail!("commitment must not be zero");
    }

    Ok(Self {
      creator,
      commitment,
      block_height,
      timestamp_ms,
    })
  }

  /// Returns the UTC timestamp associated with the inscription.
  pub fn timestamp(&self) -> Result<DateTime<Utc>> {
    let seconds = (self.timestamp_ms / 1000) as i64;
    let sub_ms = (self.timestamp_ms % 1000) as u32;
    Ok(
      Utc
        .timestamp_opt(seconds, sub_ms * 1_000_000)
        .single()
        .ok_or_else(|| anyhow::anyhow!("invalid timestamp"))?,
    )
  }
}

impl std::fmt::Display for LivingInscription {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let timestamp = self
      .timestamp()
      .map(|ts| ts.to_rfc3339())
      .unwrap_or_else(|_| "<invalid>".to_string());

    writeln!(f, "Creator      : {:?}", self.creator)?;
    writeln!(f, "Commitment   : 0x{:x}", self.commitment)?;
    writeln!(f, "Block Height : {}", self.block_height)?;
    writeln!(f, "Timestamp    : {}", timestamp)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ethers::types::H160;

  #[test]
  fn creates_valid_inscription() {
    let creator = Address::from_low_u64_be(1);
    let commitment = H256::from_low_u64_be(2);
    let inscription =
      LivingInscription::new(creator, commitment, 100, 1_600_000_000_000).expect("valid");

    assert_eq!(inscription.creator, creator);
    assert_eq!(inscription.commitment, commitment);
    assert_eq!(inscription.block_height, 100);
  }

  #[test]
  fn rejects_zero_commitment() {
    let creator = H160::repeat_byte(0x11).into();
    let result = LivingInscription::new(creator, H256::zero(), 1, 0);
    assert!(result.is_err());
  }
}
