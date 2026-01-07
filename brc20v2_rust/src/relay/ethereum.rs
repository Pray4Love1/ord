use crate::errors::Brc20Error;
use secp256k1::SecretKey;
use tracing::{debug, info};
use web3::signing::{SecretKeyRef, SigningError};
use web3::transports::Http;
use web3::types::{Address, H256, TransactionParameters, TransactionRequest, U256};
use web3::Web3;

const DEFAULT_GAS_LIMIT: u64 = 300_000;

/// Relay a proof to an Ethereum-compatible chain with offline signing.
pub async fn relay_proof_signed(
    rpc: &str,
    private_key: &SecretKey,
    contract: Address,
    data: Vec<u8>,
    value: Option<U256>,
    gas_price: Option<U256>,
    nonce: Option<U256>,
) -> Result<H256, Brc20Error> {
    info!(rpc = rpc, "creating Ethereum RPC transport");
    let transport = Http::new(rpc).map_err(|e| Brc20Error::Relay(e.to_string()))?;
    let web3 = Web3::new(transport);

    let from = SecretKeyRef::new(private_key).address();
    debug!(from = ?from, "derived sender address");

    let resolved_nonce = match nonce {
        Some(nonce) => {
            debug!(nonce = %nonce, "using provided nonce");
            nonce
        }
        None => {
            let fetched_nonce = web3
                .eth()
                .transaction_count(from, None)
                .await
                .map_err(|e| Brc20Error::Relay(e.to_string()))?;
            debug!(nonce = %fetched_nonce, "fetched nonce");
            fetched_nonce
        }
    };

    let tx = TransactionParameters {
        to: Some(contract),
        gas: U256::from(DEFAULT_GAS_LIMIT),
        gas_price,
        value: value.unwrap_or_default(),
        data: data.into(),
        nonce: Some(resolved_nonce),
        ..Default::default()
    };

    let signed_tx = web3
        .accounts()
        .sign_transaction(tx, private_key)
        .await
        .map_err(|e: SigningError| Brc20Error::Relay(e.to_string()))?;

    let tx_hash = web3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await
        .map_err(|e| Brc20Error::Relay(e.to_string()))?;

    info!(tx_hash = ?tx_hash, "relay transaction submitted");
    Ok(tx_hash)
}

/// Helper: relay with an unlocked node account.
pub async fn relay_proof_unlocked(
    rpc: &str,
    from: Address,
    contract: Address,
    data: Vec<u8>,
    gas: Option<U256>,
    gas_price: Option<U256>,
    value: Option<U256>,
    nonce: Option<U256>,
) -> Result<H256, Brc20Error> {
    info!(rpc = rpc, "creating Ethereum RPC transport (unlocked)");
    let transport = Http::new(rpc).map_err(|e| Brc20Error::Relay(e.to_string()))?;
    let web3 = Web3::new(transport);

    let tx = TransactionRequest {
        from,
        to: Some(contract),
        gas,
        gas_price,
        value,
        data: Some(data.into()),
        nonce,
        ..Default::default()
    };

    let tx_hash = web3
        .eth()
        .send_transaction(tx)
        .await
        .map_err(|e| Brc20Error::Relay(e.to_string()))?;

    info!(tx_hash = ?tx_hash, "relay transaction submitted (unlocked)");
    Ok(tx_hash)
}
