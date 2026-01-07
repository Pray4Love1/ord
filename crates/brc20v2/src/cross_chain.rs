use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use web3::{
    signing::{Key, SecretKeyRef},
    transports::Http,
    types::{Address, Bytes, TransactionRequest, H256, U256},
    Web3,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainRelay {
    pub ethereum_rpc: String,
    pub contract_address: String,
    pub private_key_hex: String,
}

impl CrossChainRelay {
    /// Create new relay config
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

    /// Build JSON payload for testing
    pub fn relay_payload(&self, inscription_id: &str) -> Result<String> {
        let payload = format!(
            "{{\"contract\":\"{}\",\"inscription\":\"{}\"}}",
            self.contract_address, inscription_id
        );
        Ok(payload)
    }

    /// Perform signed Ethereum transaction
    pub async fn send_relay_tx(&self, proof_json: &str) -> Result<H256> {
        let transport = Http::new(&self.ethereum_rpc)
            .context("Failed to create HTTP transport")?;
        let web3 = Web3::new(transport);

        // Parse private key
        let private_key_bytes = hex::decode(self.private_key_hex.trim_start_matches("0x"))
            .context("Invalid private key hex")?;
        let secret_key = SecretKeyRef::new(&private_key_bytes);

        // Parse contract address
        let contract: Address = self
            .contract_address
            .parse()
            .context("Invalid contract address")?;

        // Build tx
        let tx = TransactionRequest {
            from: secret_key.address(),
            to: Some(contract),
            gas: Some(U256::from(300_000)),
            gas_price: Some(U256::from(20_000_000_000u64)), // 20 gwei
            value: Some(U256::zero()),
            data: Some(Bytes::from(proof_json.as_bytes().to_vec())),
            nonce: None, // Let node fill in
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
