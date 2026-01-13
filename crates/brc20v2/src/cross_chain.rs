use reqwest::Client;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde_json::json;
use sha3::{Digest, Keccak256};

fn ethereum_address_from_key(private_key: &str) -> String {
    let key_bytes = hex::decode(private_key).expect("valid hex private key");
    let secret_key = SecretKey::from_slice(&key_bytes).expect("valid private key length");
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let uncompressed = public_key.serialize_uncompressed();
    let mut hasher = Keccak256::new();
    hasher.update(&uncompressed[1..]);
    let hash = hasher.finalize();
    format!("0x{}", hex::encode(&hash[12..]))
}

pub async fn relay_to_ethereum(
    proof_json: &str,
    eth_rpc: &str,
    contract: &str,
    private_key: &str,
) -> String {
    let from = ethereum_address_from_key(private_key);

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_sendTransaction",
        "params": [
            {
                "from": from,
                "to": contract,
                "gas": "0x493e0",
                "gasPrice": "0x04a817c800",
                "value": "0x0",
                "data": format!("0x{}", hex::encode(proof_json))
            }
        ]
    });

    let client = Client::new();
    let response = client
        .post(eth_rpc)
        .json(&payload)
        .send()
        .await
        .expect("relay request failed");

    let body: serde_json::Value = response.json().await.expect("valid json response");
    body.get("result")
        .and_then(|value| value.as_str())
        .unwrap_or_else(|| panic!("missing transaction hash: {body:?}"))
        .to_string()
}
