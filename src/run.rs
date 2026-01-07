use crate::brc20v2::{Brc20V2, Metadata, TransferRules, Vesting};
use crate::errors::Brc20Error;
use crate::identity::{generate_identity_commitment, IdentityProof};
use crate::inscription::Inscription;
use crate::relay::ethereum::relay_proof;
use crate::zk::ZkProof;

use serde_json::json;
use std::collections::BTreeMap;

/// Canonical execution context
pub struct ExecutionContext {
  pub block_height: u64,
  pub relayer_enabled: bool,
}

/// Entry point for a full BRC-20 v2 transfer lifecycle
pub async fn execute_transfer_e2e(
  token_id: &str,
  from: &str,
  to: &str,
  amount: u64,
  ctx: ExecutionContext,
  eth_rpc: Option<&str>,
  eth_from: Option<web3::types::Address>,
  eth_contract: Option<web3::types::Address>,
) -> Result<String, Brc20Error> {
  /* ---------------------------
   * 1. Token Metadata Assembly
   * ---------------------------
   */
  let mut vesting = BTreeMap::new();
  vesting.insert(
    from.to_string(),
    Vesting {
      unlock_block: 1_000,
    },
  );

  let metadata = Metadata {
    name: "BRC20 v2 Asset".into(),
    symbol: "BRC2".into(),
    decimals: 8,
    soulbound: false,
    vesting,
    transfer_rules: TransferRules {
      max_per_tx: Some(10_000),
      require_identity: true,
    },
  };

  let mut token = Brc20V2::new(token_id, metadata);

  /* ---------------------------
   * 2. Initial Mint
   * ---------------------------
   */
  token.mint(from, amount);

  /* ---------------------------
   * 3. SoulSync Identity Layer
   * ---------------------------
   */
  let identity: IdentityProof = generate_identity_commitment(from);

  /* ---------------------------
   * 4. State Transition + ZK
   * ---------------------------
   */
  let zk_proof: ZkProof =
    token.transfer(from, to, amount / 2, ctx.block_height, Some(&identity))?;

  /* ---------------------------
   * 5. Inscription Assembly
   * ---------------------------
   */
  let state = token.export_state();

  let inscription = Inscription::new(
    &state.token_id,
    "transfer",
    &state.prev_state_hash,
    &state.merkle_root,
    Some(&zk_proof),
  );

  let inscription_json =
    serde_json::to_string_pretty(&inscription).expect("serialization must succeed");

  /* ---------------------------
   * 6. Optional Settlement Relay
   * ---------------------------
   */
  if ctx.relayer_enabled {
    let rpc = eth_rpc.ok_or_else(|| Brc20Error::Relay("missing rpc".into()))?;
    let from_addr = eth_from.ok_or_else(|| Brc20Error::Relay("missing from".into()))?;
    let contract = eth_contract.ok_or_else(|| Brc20Error::Relay("missing contract".into()))?;

    let tx_hash = relay_proof(
      rpc,
      from_addr,
      contract,
      inscription_json.as_bytes().to_vec(),
    )
    .await?;

    return Ok(
      json!({
          "inscription": inscription_json,
          "relay_tx": format!("{:?}", tx_hash)
      })
      .to_string(),
    );
  }

  /* ---------------------------
   * 7. Return Ordinal Payload
   * ---------------------------
   */
  Ok(inscription_json)
}
