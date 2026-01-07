use sha2::{Digest, Sha256};
use web3::{
    transports::Http,
    types::{Address, Bytes, TransactionRequest, H256, U256},
    Web3,
};

/// Domain separator for cross-chain safety.
pub const RELAY_DOMAIN: &str = "BRC20V2::RELAY::ETH";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelayEnvelope {
    pub domain: String,
    pub chain_id: u64,
    pub contract: String,

    pub proof_hash: String,
    pub prev_state_hash: String,

    pub nonce: u64,

    pub source: String,
    pub timestamp: u64,

    pub calldata_hash: String,
}

fn build_calldata(envelope: &RelayEnvelope, proof_json: &str) -> Bytes {
    let payload = serde_json::json!({
        "envelope": envelope,
        "proof": proof_json,
    });

    Bytes(payload.to_string().into_bytes())
}

fn hash_calldata(data: &Bytes) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&data.0);
    hex::encode(hasher.finalize())
}

pub async fn relay_to_ethereum(
    proof_json: &str,
    proof_hash: &str,
    prev_state_hash: &str,
    nonce: u64,
    eth_rpc: &str,
    chain_id: u64,
    contract: &str,
    private_key_hex: &str,
) -> H256 {
    let transport = Http::new(eth_rpc).expect("Invalid ETH RPC");
    let web3 = Web3::new(transport);

    let private_key = hex::decode(private_key_hex).expect("Invalid private key hex");
    let secret = web3::signing::SecretKey::from_slice(&private_key)
        .expect("Invalid secp256k1 key");

    let from = secret.address();
    let to: Address = contract.parse().expect("Invalid contract address");

    let mut envelope = RelayEnvelope {
        domain: RELAY_DOMAIN.to_string(),
        chain_id,
        contract: contract.to_string(),

        proof_hash: proof_hash.to_string(),
        prev_state_hash: prev_state_hash.to_string(),

        nonce,
        source: "bitcoin".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,

        calldata_hash: String::new(),
    };

    let calldata = build_calldata(&envelope, proof_json);
    envelope.calldata_hash = hash_calldata(&calldata);

    let calldata = build_calldata(&envelope, proof_json);

    let tx = TransactionRequest {
        from,
        to: Some(to),
        gas: Some(U256::from(350_000u64)),
        max_fee_per_gas: Some(U256::from(30_000_000_000u64)),
        max_priority_fee_per_gas: Some(U256::from(2_000_000_000u64)),
        value: Some(U256::zero()),
        data: Some(calldata),
        nonce: None,
        ..Default::default()
    };

    let signed = web3
        .accounts()
        .sign_transaction(tx, &secret)
        .await
        .expect("TX signing failed");

    web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await
        .expect("TX broadcast failed")
}
