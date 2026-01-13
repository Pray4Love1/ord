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
}
