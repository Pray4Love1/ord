use serde::{Deserialize, Serialize};

use crate::brc20v2::errors::Brc20Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthereumRelay {
  pub endpoint: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayReceipt {
  pub root: [u8; 32],
  pub relay_id: String,
}

impl EthereumRelay {
  pub fn new(endpoint: impl Into<String>) -> Self {
    Self {
      endpoint: endpoint.into(),
    }
  }

  pub async fn submit_root(&self, root: [u8; 32]) -> Result<RelayReceipt, Brc20Error> {
    Ok(RelayReceipt {
      root,
      relay_id: format!("eth-relay:{}", hex::encode(root)),
    })
  }
}
