use brc20v2::zk_proof::ZkProofEnvelope;
use brc20v2::TokenState;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut balances = HashMap::new();
    balances.insert("alice".to_string(), 1000);

    let mut state = TokenState {
        token_id: "MYTOKEN".into(),
        balances,
        prev_state_hash: "0".repeat(64),
        merkle_root: "".into(),
    };

    state.transfer("alice", "bob", 100);

    let proof = ZkProofEnvelope::generate(
        "alice",
        "bob",
        100,
        &state.prev_state_hash,
        1,
        1,
        true,
    );

    let proof_json = serde_json::to_string(&proof).expect("serialize proof");

    println!("{}", proof_json);
use brc20v2::{Brc20v2, Brc20v2Token, CrossChainRelay, ZkProof, ZkProofRequest};

fn main() -> anyhow::Result<()> {
  let token = Brc20v2Token {
    ticker: "ORD".to_string(),
    max_supply: 21_000_000,
    decimals: 8,
  };
  let brc20 = Brc20v2::new(token);

  let mint_payload = brc20.mint(1_000, "tb1qrecipient")?;
  let transfer = brc20.transfer(250, "tb1qrecipient")?;
  let proof = ZkProof::generate(&ZkProofRequest {
    statement: "transfer".to_string(),
    witness: format!("{}:{}", transfer.ticker, transfer.amount),
  })?;
  let inscription_id = brc20.create_inscription(&proof.proof);

  let relay = CrossChainRelay::new("http://127.0.0.1:8545", "0x0000000000000000");
  let relay_payload = relay.relay(&inscription_id)?;

  println!("mint: {mint_payload}");
  println!("transfer: {:?}", transfer);
  println!("proof: {}", proof.proof);
  println!("inscription: {inscription_id}");
  println!("relay: {relay_payload}");

  Ok(())
}
