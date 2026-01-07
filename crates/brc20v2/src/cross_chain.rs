use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainRelay {
  pub ethereum_rpc: String,
  pub contract_address: String,
}

impl CrossChainRelay {
  pub fn new(ethereum_rpc: impl Into<String>, contract_address: impl Into<String>) -> Self {
    Self {
      ethereum_rpc: ethereum_rpc.into(),
      contract_address: contract_address.into(),
    }
  }

  pub fn relay(&self, inscription_id: &str) -> Result<String> {
    let payload = format!(
      "{{\"contract\":\"{}\",\"inscription\":\"{}\"}}",
      self.contract_address, inscription_id
    );
    Ok(payload)
  }
}
