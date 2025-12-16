use web3::types::{TransactionRequest, U256};
use web3::transports::Http;
use web3::Web3;

pub async fn relay_to_ethereum(proof_json: &str, eth_rpc: &str, contract: &str, private_key: &str) -> web3::types::H256 {
    let transport = Http::new(eth_rpc).unwrap();
    let web3 = Web3::new(transport);

    let tx = TransactionRequest {
        from: web3::signing::SecretKey::from_slice(&hex::decode(private_key).unwrap()).address(),
        to: Some(contract.parse().unwrap()),
        gas: Some(U256::from(300_000)),
        gas_price: Some(U256::from(20_000_000_000u64)),
        value: Some(U256::zero()),
        data: Some(proof_json.as_bytes().into()),
        nonce: None,
        ..Default::default()
    };
    let tx_hash = web3.eth().send_transaction(tx).await.unwrap();
    tx_hash
}
