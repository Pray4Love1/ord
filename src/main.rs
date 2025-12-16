mod brc20v2;
mod zk_proof;
mod cross_chain;
mod config;

use crate::brc20v2::BRC20v2;
use crate::config::Config;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::Write;
use tokio::runtime::Runtime;

fn main() {
    let config = Config::load("config.json");

    let mut token = BRC20v2::new("MYTOKEN");

    let metadata = crate::brc20v2::Metadata {
        name: "MyToken".to_string(),
        symbol: "MTK".to_string(),
        decimals: 8,
        soulbound: false,
        transfer_rules: [("max_per_tx".to_string(), 1000u64)].iter().cloned().collect(),
        vesting: [("bc1qalice...".to_string(), 820000u64)].iter().cloned().collect()
    };

    token.set_metadata(metadata);

    // Mint
    token.mint("bc1qalice...", 1000);
    token.mint("bc1qbob...", 500);
    let mint_inscription = token.generate_inscription("mint", None);
    let mut file = File::create("mint_inscription.json").unwrap();
    file.write_all(to_string_pretty(&mint_inscription).unwrap().as_bytes()).unwrap();

    // Transfer with zk proof
    let proof = token.transfer("bc1qalice...", "bc1qbob...", 200, 820001, true);
    let transfer_inscription = token.generate_inscription("transfer", Some(proof));
    let mut file2 = File::create("transfer_inscription.json").unwrap();
    file2.write_all(to_string_pretty(&transfer_inscription).unwrap().as_bytes()).unwrap();

    // Broadcast to Bitcoin using ord CLI (external)
    std::process::Command::new("ord")
        .args(&["inscribe", "mint_inscription.json", "--fee-rate", &config.fee_per_byte.to_string(), "--rpc-url", &config.rpc_url])
        .output().expect("Failed to broadcast mint");

    std::process::Command::new("ord")
        .args(&["inscribe", "transfer_inscription.json", "--fee-rate", &config.fee_per_byte.to_string(), "--rpc-url", &config.rpc_url])
        .output().expect("Failed to broadcast transfer");

    // Cross-chain relay
    let rt = Runtime::new().unwrap();
    let relay_tx = rt.block_on(cross_chain::relay_to_ethereum(
        &serde_json::to_string(&transfer_inscription).unwrap(),
        &config.eth_rpc,
        &config.eth_contract,
        &config.eth_private_key
    ));
    println!("Relay tx hash: {:?}", relay_tx);
}
