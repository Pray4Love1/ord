mod cross_chain;
mod zk_proof;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub use crate::cross_chain::CrossChainRelay;
pub use crate::zk_proof::{ZkProof, ZkProofRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brc20v2Token {
  pub ticker: String,
  pub max_supply: u128,
  pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brc20v2Transfer {
  pub ticker: String,
  pub amount: u128,
  pub to: String,
}

pub struct Brc20v2 {
  token: Brc20v2Token,
}

impl Brc20v2 {
  pub fn new(token: Brc20v2Token) -> Self {
    Self { token }
  }

  pub fn mint(&self, amount: u128, recipient: &str) -> Result<String> {
    let transfer = Brc20v2Transfer {
      ticker: self.token.ticker.clone(),
      amount,
      to: recipient.to_string(),
    };
    let payload = serde_json::to_vec(&transfer)?;
    Ok(hex::encode(payload))
  }

  pub fn transfer(&self, amount: u128, recipient: &str) -> Result<Brc20v2Transfer> {
    Ok(Brc20v2Transfer {
      ticker: self.token.ticker.clone(),
      amount,
      to: recipient.to_string(),
    })
  }

  pub fn create_inscription(&self, memo: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(self.token.ticker.as_bytes());
    hasher.update(memo.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
  }
}
