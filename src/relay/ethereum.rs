use web3::{types::{TransactionRequest, Address, U256, H256}, Web3};
use web3::transports::Http;
use crate::errors::Brc20Error;
use web3::signing::{SecretKeyRef, SigningError};
use secp256k1::SecretKey;

/// Relay a proof to an Ethereum-compatible chain
pub async fn relay_proof_signed(
    rpc: &str,
    private_key: &SecretKey,
    contract: Address,
    data: Vec<u8>,
    value: Option<U256>,
    gas_price: Option<U256>,
) -> Result<H256, Brc20Error> {
    // Create HTTP transport
    let transport = Http::new(rpc).map_err(|e| Brc20Error::Relay(e.to_string()))?;
    let web3 = Web3::new(transport);

    // Derive sender address
    let from = SecretKeyRef::new(private_key).address();

    // Get nonce
    let nonce = web3.eth()
        .transaction_count(from, None)
        .await
        .map_err(|e| Brc20Error::Relay(e.to_string()))?;

    // Prepare transaction
    let tx = TransactionRequest {
        from,
        to: Some(contract),
        gas: Some(300_000.into()),
        gas_price,
        value,
        data: Some(data.into()),
        nonce: Some(nonce),
        ..Default::default()
    };

    // Sign the transaction
    let signed_tx = web3.accounts()
        .sign_transaction(tx, private_key)
        .await
        .map_err(|e: SigningError| Brc20Error::Relay(e.to_string()))?;

    // Send raw transaction
    let tx_hash = web3.eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await
        .map_err(|e| Brc20Error::Relay(e.to_string()))?;

    Ok(tx_hash)
}

/// Helper: simple relay without signing (relies on unlocked node)
pub async fn relay_proof_unlocked(
    rpc: &str,
    from: Address,
    contract: Address,
    data: Vec<u8>,
    gas: Option<U256>,
) -> Result<H256, Brc20Error> {
    let transport = Http::new(rpc).map_err(|e| Brc20Error::Relay(e.to_string()))?;
    let web3 = Web3::new(transport);

    let tx = TransactionRequest {
        from,
        to: Some(contract),
        gas,
        data: Some(data.into()),
        ..Default::default()
    };

    web3.eth()
        .send_transaction(tx)
        .await
        .map_err(|e| Brc20Error::Relay(e.to_string()))
}
