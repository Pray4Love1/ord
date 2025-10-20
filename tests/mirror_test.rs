//! Mirror Test Harness — Bitcoin → Rust Bridge → EVM MirrorVerifier
//! Run with: cargo test -- --nocapture
//! Requires: ethers = "2", serde_json, chrono, blake3, k256

use blake3;
use chrono::Utc;
use ethers::{
    prelude::*,
    types::{Address, Bytes},
    utils::keccak256,
};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use std::sync::Arc;

mod living_inscription;
mod mirror_bridge;

use living_inscription::*;
use mirror_bridge::*;

#[tokio::test]
async fn simulate_full_flow() -> anyhow::Result<()> {
    // Create ephemeral signer for testing
    let signer = SigningKey::random(&mut rand::thread_rng());
    let pubkey = signer.verifying_key();
    let creator_addr = Address::from_slice(&keccak256(pubkey.to_encoded_point(false).as_bytes())[12..]);

    // Build a simple living inscription
    let core = InscriptionCore {
        version: 1,
        parent_hash: None,
        creator: format!("{:?}", creator_addr),
        timestamp: Utc::now(),
        content_uri: "ipfs://QmGenesis".into(),
        metadata: serde_json::json!({"type":"artifact","series":1}),
    };

    let state = InscriptionState {
        block_height: 830_000,
        external_entropy: Some("randomhash123".into()),
        mood: Some("dawn".into()),
        mirror_hash: None,
    };

    let mut ins = LivingInscription {
        core,
        state,
        signature: String::new(),
    };

    // Compute commitment and sign
    let commitment = ins.commitment();
    let msg_hash = blake3::hash(commitment.as_bytes());
    let sig: Signature = signer.sign(msg_hash.as_bytes());
    ins.signature = format!("0x{}", hex::encode(sig.to_bytes()));

    println!("🪶 Living inscription commitment: {}", commitment);
    println!("Creator address: {:?}", creator_addr);

    // Build the mirror proof
    let proof = build_proof(&ins, "ethereum-sepolia");
    println!("🪞 Mirror proof: {}", serde_json::to_string_pretty(&proof)?);

    // Connect to local JSON-RPC (e.g., Anvil)
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let chain_id = provider.get_chainid().await?;
    println!("Connected to chain id {}", chain_id);

    // Assume MirrorVerifier is deployed locally
    let contract_addr: Address = "0x1111111111111111111111111111111111111111"
        .parse()
        .unwrap();

    // Encode proof call data
    let abi = include_str!("./MirrorVerifier.abi.json");
    let contract = Contract::new(contract_addr, abi.parse::<Abi>()?, Arc::new(provider));

    let sig_bytes = Bytes::from(hex::decode(&ins.signature.trim_start_matches("0x"))?);
    let call = contract.method::<_, H256>(
        "postMirror",
        (
            creator_addr,
            H256::from_slice(&blake3::hash(commitment.as_bytes()).as_bytes()[..32]),
            proof.block_height,
            proof.timestamp_ms,
            sig_bytes,
        ),
    )?;

    let tx = call.tx;
    println!("Simulated tx data: 0x{}", hex::encode(tx.data().unwrap()));

    // Here you could send with a local wallet to finalize the test
    Ok(())
}
