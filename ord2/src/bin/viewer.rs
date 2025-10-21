```rust
use anyhow::{Context, Result};
use ethers::types::H256;
use ord2::{living_inscription::LivingInscription, mirror_bridge::MirrorBridge};

fn parse_commitment(arg: &str) -> Result<H256> {
  let trimmed = arg.trim_start_matches("0x");
  let bytes = hex::decode(trimmed).context("commitment must be hex")?;
  if bytes.len() != 32 {
    anyhow::bail!("commitment must be 32 bytes (64 hex chars)");
  }
  Ok(H256::from_slice(&bytes))
}

fn main() -> Result<()> {
  let abi = include_str!("../../abi/MirrorVerifier.abi.json");
  let verifier = std::env::var("VERIFIER_ADDR")
    .unwrap_or_else(|_| "0x1111111111111111111111111111111111111111".into())
    .parse()
    .context("bad verifier address")?;
  let rpc = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8545".into());

  let commitment_arg = std::env::args()
    .nth(1)
    .context("usage: viewer <commitment>")?;
  let commitment = parse_commitment(&commitment_arg)?;

  let bridge = MirrorBridge::new(&rpc, verifier, abi)?;
  let inscription = bridge.fetch_inscription_blocking(commitment)?;

  print_json(&inscription)?;
  Ok(())
}

fn print_json(inscription: &LivingInscription) -> Result<()> {
  let ts = inscription.timestamp()?;
  let json = serde_json::json!({
      "creator": format!("{:?}", inscription.creator),
      "block_height": inscription.block_height,
      "timestamp": ts.to_rfc3339(),
      "commitment": format!("0x{:x}", inscription.commitment),
  });

  println!("{}", serde_json::to_string_pretty(&json)?);
  Ok(())
}
```
