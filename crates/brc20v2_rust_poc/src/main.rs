mod bitcoin;
mod cross_chain;
mod inscription;
mod merkle;
mod state;
mod zk;

use chrono::Utc;
use inscription::Inscription;
use state::TokenState;
use zk::identity::{IdentityVerifier, SoulSyncVerifier};
use zk::proof::ZkTransferProof;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut token = TokenState::new("MYTOKEN");

    token.mint("bc1qalice", 1_000);
    token.mint("bc1qbob", 500);

    let verifier = SoulSyncVerifier;
    if !verifier.verify("bc1qalice") {
        anyhow::bail!("identity not verified");
    }

    let prev = token.compute_state_hash();
    token.transfer("bc1qalice", "bc1qbob", 200)?;

    let leaves = token
        .balances
        .iter()
        .map(|(key, value)| format!("{key}:{value}"))
        .collect::<Vec<_>>();
    token.merkle_root = merkle::merkle_root(&leaves);
    token.prev_state_hash = token.compute_state_hash();

    let proof = ZkTransferProof::generate("bc1qalice", "bc1qbob", 200, &prev);

    let inscription = Inscription {
        protocol: "brc20v2",
        action: "transfer".into(),
        token_id: token.token_id.clone(),
        state_hash: token.prev_state_hash.clone(),
        merkle_root: token.merkle_root.clone(),
        proof: Some(serde_json::to_value(&proof)?),
        timestamp: Utc::now().timestamp(),
    };

    println!("{}", serde_json::to_string_pretty(&inscription)?);

    Ok(())
}
