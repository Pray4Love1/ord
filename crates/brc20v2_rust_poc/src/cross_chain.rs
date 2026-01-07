use web3::transports::Http;
use web3::types::{TransactionRequest, U256};
use web3::Web3;

pub async fn relay(
    rpc: &str,
    contract: &str,
    from: web3::types::Address,
    data: Vec<u8>,
) -> web3::Result<web3::types::H256> {
    let web3 = Web3::new(Http::new(rpc)?);

    let tx = TransactionRequest {
        from,
        to: Some(contract.parse().unwrap()),
        gas: Some(U256::from(300_000)),
        data: Some(data.into()),
        ..Default::default()
    };

    web3.eth().send_transaction(tx).await
}
