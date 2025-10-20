//! Living Inscription Watcher Daemon v0.1
//! Watches Bitcoin blocks, extracts target inscriptions, and mirrors proofs to EVM chains.
//! Rust 2024 edition

use std::time::Duration;

use anyhow::{Result, anyhow};
use chrono::Utc;
use ord::{
  living_inscription::{InscriptionCore, InscriptionState, LivingInscription},
  mirror_bridge::{build_proof, post_proof},
};
use reqwest::Client;
use sha3::{Digest, Sha3_256};
use tokio::time::sleep;

#[derive(Clone)]
pub struct WatcherConfig {
  pub bitcoin_rpc: String, // http://127.0.0.1:8332
  pub rpc_user: String,
  pub rpc_pass: String,
  pub mirror_rpc: String, // http://localhost:8545
  pub verifier_address: String,
  pub poll_interval: u64, // seconds
}

pub struct Watcher {
  cfg: WatcherConfig,
  http: Client,
}

impl Watcher {
  pub fn new(cfg: WatcherConfig) -> Result<Self> {
    let http = Client::builder().build()?;
    Ok(Self { cfg, http })
  }

  pub async fn run(&self) -> Result<()> {
    println!("🔭 Starting Living Inscription Watcher...");
    loop {
      if let Err(e) = self.scan_block().await {
        eprintln!("⚠️  Scan error: {:?}", e);
      }
      sleep(Duration::from_secs(self.cfg.poll_interval)).await;
    }
  }

  async fn scan_block(&self) -> Result<()> {
    // Query latest block height
    let body = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "blockheight",
        "method": "getblockcount",
        "params": []
    });
    let resp = self
      .http
      .post(&self.cfg.bitcoin_rpc)
      .basic_auth(&self.cfg.rpc_user, Some(&self.cfg.rpc_pass))
      .json(&body)
      .send()
      .await?
      .json::<serde_json::Value>()
      .await?;

    if let Some(error) = resp.get("error").filter(|error| !error.is_null()) {
      return Err(anyhow!("bitcoin RPC getblockcount error: {error}"));
    }

    let height = resp
      .get("result")
      .and_then(|value| value.as_u64())
      .ok_or_else(|| anyhow!("bitcoin RPC getblockcount missing numeric result"))?;
    println!("⛓️  Checking Bitcoin block {height}");

    // Replace this stub with real Ordinal detection logic.
    if height % 100 == 0 {
      println!("📜 New simulated inscription detected at block {height}");
      self.handle_new_inscription(height).await?;
    }

    Ok(())
  }

  async fn handle_new_inscription(&self, height: u64) -> Result<()> {
    // Build synthetic inscription for demo
    let core = InscriptionCore {
      version: 1,
      parent_hash: None,
      creator: "bc1qWatcherExample".into(),
      timestamp: Utc::now(),
      content_uri: format!("ipfs://QmAuto{height}"),
      metadata: serde_json::json!({"type":"auto","height":height}),
    };
    let state = InscriptionState {
      block_height: height,
      external_entropy: Some(hex::encode(Sha3_256::digest(height.to_string().as_bytes()))),
      mood: Some("emergent".into()),
      mirror_hash: None,
    };
    let ins = LivingInscription {
      core,
      state,
      signature: "0xautowatcher".into(),
    };
    let proof = build_proof(&ins, "ethereum-sepolia");

    println!("🚀 Broadcasting mirror proof...");
    post_proof(
      &self.http,
      &proof,
      &self.cfg.mirror_rpc,
      &self.cfg.verifier_address,
    )
    .await?;
    Ok(())
  }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let cfg = WatcherConfig {
    bitcoin_rpc: "http://127.0.0.1:8332".into(),
    rpc_user: "user".into(),
    rpc_pass: "pass".into(),
    mirror_rpc: "http://localhost:8545".into(),
    verifier_address: "0x1111111111111111111111111111111111111111".into(),
    poll_interval: 10,
  };
  let watcher = Watcher::new(cfg)?;
  watcher.run().await
}
