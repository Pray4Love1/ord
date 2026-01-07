use web3::signing::SecretKeyRef;
use web3::types::{Bytes, TransactionRequest, H256, U256};
use web3::{transports::Http, Web3};

pub async fn relay_to_ethereum(
    proof_json: &str,
    eth_rpc: &str,
    contract: &str,
    private_key: &str,
) -> web3::Result<H256> {
    let transport = Http::new(eth_rpc)?;
    let web3 = Web3::new(transport);
    let secret_key = SecretKeyRef::new(&hex::decode(private_key).map_err(|err| {
        web3::Error::Decoder(format!("invalid private key: {err}"))
    })?);

    let tx = TransactionRequest {
        from: secret_key.address(),
        to: Some(contract.parse().map_err(|err| {
            web3::Error::Decoder(format!("invalid contract address: {err}"))
        })?),
        gas: Some(U256::from(300_000)),
        gas_price: Some(U256::from(20_000_000_000u64)),
        value: Some(U256::zero()),
        data: Some(Bytes::from(proof_json.as_bytes().to_vec())),
        nonce: None,
        ..Default::default()
    };

    web3.eth().send_transaction(tx).await
}
