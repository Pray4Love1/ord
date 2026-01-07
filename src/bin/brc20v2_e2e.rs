use brc20v2::cross_chain;
use brc20v2::{BRC20v2, Metadata};
use serde::Deserialize;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::Command;
use tokio::runtime::Runtime;

#[derive(Deserialize)]
struct Config {
    fee_per_byte: u64,
    rpc_url: String,
    eth_rpc: String,
    eth_contract: String,
    eth_private_key: String,
}

impl Config {
    fn load(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read config file");
        serde_json::from_str(&contents).expect("Failed to parse config file")
    }
}

fn main() {
    let config = Config::load("config.json");

    let mut token = BRC20v2::new("MYTOKEN");

    let metadata = Metadata {
        name: "MyToken".to_string(),
        symbol: "MTK".to_string(),
        decimals: 8,
        soulbound: false,
        transfer_rules: HashMap::from([("max_per_tx".to_string(), 1000u64)]),
        vesting: HashMap::from([("bc1qalice...".to_string(), 820000u64)]),
    };

    token.set_metadata(metadata);

    token.mint("bc1qalice...", 1000);
    token.mint("bc1qbob...", 500);
    let mint_inscription = token.generate_inscription("mint", None);
    let mut file = fs::File::create("mint_inscription.json").expect("Failed to create mint file");
    file.write_all(
        to_string_pretty(&mint_inscription)
            .expect("Failed to serialize mint inscription")
            .as_bytes(),
    )
    .expect("Failed to write mint inscription");

    let proof = token.transfer("bc1qalice...", "bc1qbob...", 200, 820001, true);
    let transfer_inscription = token.generate_inscription("transfer", Some(proof));
    let mut file2 =
        fs::File::create("transfer_inscription.json").expect("Failed to create transfer file");
    file2
        .write_all(
            to_string_pretty(&transfer_inscription)
                .expect("Failed to serialize transfer inscription")
                .as_bytes(),
        )
        .expect("Failed to write transfer inscription");

    Command::new("ord")
        .args([
            "inscribe",
            "mint_inscription.json",
            "--fee-rate",
            &config.fee_per_byte.to_string(),
            "--rpc-url",
            &config.rpc_url,
        ])
        .output()
        .expect("Failed to broadcast mint");

    Command::new("ord")
        .args([
            "inscribe",
            "transfer_inscription.json",
            "--fee-rate",
            &config.fee_per_byte.to_string(),
            "--rpc-url",
            &config.rpc_url,
        ])
        .output()
        .expect("Failed to broadcast transfer");

    let rt = Runtime::new().expect("Failed to build tokio runtime");
    let relay_tx = rt
        .block_on(cross_chain::relay_to_ethereum(
            &serde_json::to_string(&transfer_inscription)
                .expect("Failed to serialize relay payload"),
            &config.eth_rpc,
            &config.eth_contract,
            &config.eth_private_key,
        ))
        .expect("Failed to relay to Ethereum");
    println!("Relay tx hash: {relay_tx:?}");
}
