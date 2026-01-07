use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub network: String,
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_pass: String,
    pub fee_per_byte: u64,
    pub eth_rpc: String,
    pub eth_contract: String,
    pub eth_private_key: String,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Unable to read config.json");
        serde_json::from_str(&data).expect("Invalid config format")
    }
}
