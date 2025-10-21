```rust
use std::sync::Arc;
use anyhow::Result;
use ethers::{
  abi::Abi,
  contract::Contract,
  providers::{Http, Middleware, Provider},
  types::{Address, H256},
};
use tokio::runtime::Runtime;

use crate::living_inscription::LivingInscription;

/// Bridge for interacting with the on-chain MirrorVerifier contract.
pub struct MirrorBridge {
  contract: Contract<Provider<Http>>,
}

impl MirrorBridge {
  /// Creates a new bridge by parsing the ABI JSON and creating a contract handle.
  pub fn new(rpc_url: &str, verifier: Address, abi_json: &str) -> Result<Self> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = provider.with_sender(verifier);
    let provider = Arc::new(provider);
    let abi: Abi = serde_json::from_str(abi_json)?;
    Ok(Self {
      contract: Contract::new(verifier, abi, provider),
    })
  }

  /// Fetches an inscription by its commitment key.
  pub async fn fetch_inscription(&self, commitment: H256) -> Result<LivingInscription> {
    let (creator, _commitment, block_height, timestamp_ms): (Address, [u8; 32], u64, u64) =
      self.contract.method("mirrors", commitment)?.call().await?;

    LivingInscription::new(creator, commitment, block_height, timestamp_ms)
  }

  /// Blocking convenience wrapper for [`fetch_inscription`].
  pub fn fetch_inscription_blocking(&self, commitment: H256) -> Result<LivingInscription> {
    let rt = Runtime::new()?;
    rt.block_on(self.fetch_inscription(commitment))
  }

  pub fn provider(&self) -> Arc<Provider<Http>> {
    self.contract.client().clone()
  }
}
```
