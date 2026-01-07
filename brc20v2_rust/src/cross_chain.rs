use web3::transports::Http;
use web3::types::{TransactionRequest, H256, U256};
use web3::Web3;

pub async fn relay_to_ethereum(
    proof_json: &str,
    eth_rpc: &str,
    contract: &str,
    private_key: &str,
) -> H256 {
    let transport = Http::new(eth_rpc).expect("Failed to connect to Ethereum RPC");
    let web3 = Web3::new(transport);

    let from_address = web3::signing::SecretKey::from_slice(
        &hex::decode(private_key).expect("Invalid private key hex"),
    )
    .expect("Invalid private key")
    .address();

    let tx = TransactionRequest {
        from: from_address,
        to: Some(contract.parse().expect("Invalid contract address")),
        gas: Some(U256::from(300_000)),
        gas_price: Some(U256::from(20_000_000_000u64)),
        value: Some(U256::zero()),
        data: Some(proof_json.as_bytes().to_vec().into()),
        nonce: None,
        ..Default::default()
    };

    web3.eth().send_transaction(tx).await.expect("Relay failed")
}
