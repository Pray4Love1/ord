use sha2::{Digest, Sha256};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use web3::{
    signing::{Key, SecretKeyRef},
    transports::Http,
    types::{Address, Bytes, TransactionRequest, H256, U256},
    Web3,
};

/// Domain separator for Ethereum relay scope
pub const RELAY_DOMAIN: &str = "BRC20V2::RELAY::ETH";

/// Relay envelope for calldata + proof relay
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Manual calldata serializer (for sending to EVM contract)
fn build_calldata(envelope: &RelayEnvelope, proof_json: &str) -> Bytes {
    let payload = serde_json::json!({
        "envelope": envelope,
        "proof": proof_json,
    });

    Bytes(payload.to_string().into_bytes())
}

/// Hash the full calldata payload for integrity
fn hash_calldata(data: &Bytes) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&data.0);
    hex::encode(hasher.finalize())
}

/// Full Ethereum relay for advanced proof envelope
pub async fn relay_to_ethereum(
    proof_json: &str,
    proof_hash: &str,
    prev_state_hash: &str,
    nonce: u64,
    eth_rpc: &str,
    chain_id: u64,
    contract: &str,
    private_key_hex: &str,
) -> Result<H256> {
    let transport = Http::new(eth_rpc).context("Invalid ETH RPC URL")?;
    let web3 = Web3::new(transport);

    let private_key = hex::decode(private_key_hex.trim_start_matches("0x"))
        .context("Invalid private key hex")?;
    let secret = web3::signing::SecretKey::from_slice(&private_key)
        .context("Invalid secp256k1 private key")?;

    let from = secret.address();
    let to: Address = contract.parse().context("Invalid contract address")?;

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
        gas: Some(U256::from(350_000)),
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
        .context("Failed to sign relay transaction")?;

    let tx_hash = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await
        .context("Failed to broadcast transaction")?;

    Ok(tx_hash)
}

/// Simpler structured relay for basic use/testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainRelay {
    pub ethereum_rpc: String,
    pub contract_address: String,
    pub private_key_hex: String,
}

impl CrossChainRelay {
    pub fn new(
        ethereum_rpc: impl Into<String>,
        contract_address: impl Into<String>,
        private_key_hex: impl Into<String>,
    ) -> Self {
        Self {
            ethereum_rpc: ethereum_rpc.into(),
            contract_address: contract_address.into(),
            private_key_hex: private_key_hex.into(),
        }
    }

    pub fn relay_payload(&self, inscription_id: &str) -> Result<String> {
        let payload = format!(
            "{{\"contract\":\"{}\",\"inscription\":\"{}\"}}",
            self.contract_address, inscription_id
        );
        Ok(payload)
    }

    pub async fn send_relay_tx(&self, proof_json: &str) -> Result<H256> {
        let transport = Http::new(&self.ethereum_rpc)
            .context("Failed to create HTTP transport")?;
        let web3 = Web3::new(transport);

        let private_key_bytes = hex::decode(self.private_key_hex.trim_start_matches("0x"))
            .context("Invalid private key hex")?;
        let secret_key = SecretKeyRef::new(&private_key_bytes);

        let contract: Address = self
            .contract_address
            .parse()
            .context("Invalid contract address")?;

        let tx = TransactionRequest {
            from: secret_key.address(),
            to: Some(contract),
            gas: Some(U256::from(300_000)),
            gas_price: Some(U256::from(20_000_000_000u64)),
            value: Some(U256::zero()),
            data: Some(Bytes::from(proof_json.as_bytes().to_vec())),
            nonce: None,
            ..Default::default()
        };

        let tx_hash = web3
            .eth()
            .send_transaction(tx)
            .await
            .context("Failed to send Ethereum relay transaction")?;

        Ok(tx_hash)
    }
}
