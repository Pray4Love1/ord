use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// Network selector (Bitcoin side)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

/// Ethereum settlement config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumConfig {
    pub rpc_url: String,
    pub chain_id: u64,
    pub contract_address: String,

    /// hex-encoded private key (NO 0x prefix)
    pub private_key: String,

    /// replay protection
    pub relay_nonce_start: u64,
}

/// Ord / Bitcoin RPC config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub network: BitcoinNetwork,
    pub rpc_url: String,
    pub ord_binary: String,

    /// sats/vbyte
    pub fee_rate: u64,

    /// protocol safety
    pub min_confirmations: u32,
}

/// Protocol-level settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub chain_id: String,
    pub require_identity: bool,
    pub max_inscription_bytes: usize,
}

/// Root config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bitcoin: BitcoinConfig,
    pub ethereum: EthereumConfig,
    pub protocol: ProtocolConfig,
}

impl Config {
    /// Load and validate config from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Config file not found: {:?}", path.as_ref()));

        let cfg: Config = serde_json::from_str(&raw).expect("Invalid config.json format");

        cfg.validate();
        cfg
    }

    /// Hard protocol safety checks
    fn validate(&self) {
        // ---- Bitcoin checks ----
        if self.bitcoin.fee_rate == 0 {
            panic!("fee_rate must be > 0");
        }

        if self.bitcoin.ord_binary.is_empty() {
            panic!("ord_binary path must be set");
        }

        // ---- Ethereum checks ----
        if self.ethereum.private_key.len() != 64 {
            panic!("Ethereum private_key must be 32-byte hex");
        }

        if !self.ethereum.rpc_url.starts_with("http") {
            panic!("Invalid Ethereum RPC URL");
        }

        // ---- Protocol checks ----
        if self.protocol.chain_id.is_empty() {
            panic!("protocol.chain_id must not be empty");
        }

        if self.protocol.max_inscription_bytes < 100 {
            panic!("max_inscription_bytes too small");
        }
    }

    /// Helper: is Bitcoin mainnet?
    pub fn is_mainnet(&self) -> bool {
        matches!(self.bitcoin.network, BitcoinNetwork::Mainnet)
    }

    /// Helper: ord inscribe command base
    pub fn ord_base_args(&self) -> Vec<String> {
        vec![
            self.bitcoin.ord_binary.clone(),
            "inscribe".to_string(),
            "--fee-rate".to_string(),
            self.bitcoin.fee_rate.to_string(),
            "--rpc-url".to_string(),
            self.bitcoin.rpc_url.clone(),
        ]
    }
}
