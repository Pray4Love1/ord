use web3::transports::Http;
use web3::{types::TransactionRequest, Web3};

use crate::errors::Brc20Error;

pub async fn relay_proof(
    rpc: &str,
    from: web3::types::Address,
    contract: web3::types::Address,
    data: Vec<u8>,
) -> Result<web3::types::H256, Brc20Error> {
    let transport = Http::new(rpc).map_err(|e| Brc20Error::Relay(e.to_string()))?;
    let web3 = Web3::new(transport);

    let tx = TransactionRequest {
        from,
        to: Some(contract),
        gas: Some(300_000.into()),
        data: Some(data.into()),
        ..Default::default()
    };

    web3
        .eth()
        .send_transaction(tx)
        .await
        .map_err(|e| Brc20Error::Relay(e.to_string()))
}
