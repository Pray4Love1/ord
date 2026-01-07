use crate::errors::Brc20Error;
use crate::relay::ethereum::relay_proof_signed;
use secp256k1::SecretKey;
use std::str::FromStr;
use tracing::info;
use web3::types::{Address, H256, U256};

pub async fn relay_to_ethereum(
    proof_json: &str,
    eth_rpc: &str,
    contract: &str,
    private_key: &str,
) -> Result<H256, Brc20Error> {
    info!(rpc = eth_rpc, "preparing Ethereum relay");

    let contract_address = Address::from_str(contract)
        .map_err(|e| Brc20Error::Relay(format!("Invalid contract address: {e}")))?;

    let private_key_bytes =
        hex::decode(private_key).map_err(|e| Brc20Error::Relay(format!("Invalid key hex: {e}")))?;
    let private_key = SecretKey::from_slice(&private_key_bytes)
        .map_err(|e| Brc20Error::Relay(format!("Invalid private key: {e}")))?;

    relay_proof_signed(
        eth_rpc,
        &private_key,
        contract_address,
        proof_json.as_bytes().to_vec(),
        Some(U256::zero()),
        None,
        None,
    )
    .await
}
