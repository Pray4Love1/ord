//! Living Inscription Viewer v0.1
//! Reconstructs lineage and mirror proofs for any inscription.
//! Run with: cargo run --bin viewer <commitment_hex>

use std::env;
use std::sync::Arc;

use anyhow::Context;
use chrono::{DateTime, Utc};
use ethers::{abi::Abi, prelude::*};
use ord::living_inscription::InscriptionCore;
use ord::mirror_bridge::MirrorRecord;
use serde_json::json;

/// Simple viewer structure
pub struct Viewer {
  provider: Arc<Provider<Http>>,
  contract: Contract<Arc<Provider<Http>>>,
}

impl Viewer {
  pub async fn new(rpc_url: &str, verifier_address: Address) -> anyhow::Result<Self> {
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url).context("invalid RPC URL")?);
    let abi = include_str!("./MirrorVerifier.abi.json");
    let contract = Contract::new(verifier_address, abi.parse::<Abi>()?, provider.clone());
    Ok(Self { provider, contract })
  }

  /// Query a mirror by commitment hash
  pub async fn get_mirror(&self, commitment_hex: &str) -> anyhow::Result<()> {
    let normalized = commitment_hex.trim_start_matches("0x");
    let key = H256::from_slice(&hex::decode(normalized).context("invalid commitment hex")?);
    let record: (Address, [u8; 32], u64, u64) =
      self.contract.method("mirrors", key)?.call().await?;
    let record = MirrorRecord::from(record);

    let timestamp: DateTime<Utc> =
      DateTime::from_timestamp_millis(record.timestamp_ms as i64).unwrap_or_else(|| Utc::now());

    let summary = json!({
      "creator": format!("{:#x}", record.creator),
      "block_height": record.block_height,
      "timestamp": timestamp.to_rfc3339(),
      "commitment": format!("0x{normalized}"),
    });

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
  }

  /// Walk an inscription lineage by following parent hashes
  pub async fn walk_lineage(&self, root: &InscriptionCore) -> anyhow::Result<()> {
    println!("🔗 Lineage traversal starting from {}", root.content_uri);
    let mut current = root.parent_hash.clone();
    let mut depth = 0;

    while let Some(hash) = current {
      depth += 1;
      println!("{}↳ Parent {}: {}", " ".repeat(depth * 2), depth, hash);

      if depth > 3 {
        break;
      }

      current = None;
    }

    Ok(())
  }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: viewer <commitment_hex>");
    std::process::exit(1);
  }

  let commitment = &args[1];
  let rpc = "http://localhost:8545";
  let verifier: Address = "0x1111111111111111111111111111111111111111".parse()?;

  let viewer = Viewer::new(rpc, verifier).await?;
  viewer.get_mirror(commitment).await?;
  Ok(())
}
