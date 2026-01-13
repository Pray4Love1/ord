use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub bitcoin_network: String,
    pub eth_rpc: String,
    pub eth_contract: String,
    pub eth_private_key: String,
    pub chain_id: u32,
}
