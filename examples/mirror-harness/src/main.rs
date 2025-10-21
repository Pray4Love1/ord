use std::sync::Arc;

use blake3::Hasher;
use ethers::abi::{encode_packed, Token};
use ethers::contract::{abigen, ContractFactory};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{Signer, SignerMiddleware, Wallet};
use ethers::types::{Bytes, H256, U256};
use ethers::utils::{keccak256, Anvil};
use ethers_solc::{artifacts::output::ProjectCompileOutput, Project, ProjectPathsConfig, Solc};
use serde::Serialize;
use tempfile::tempdir;

type Client = Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

abigen!(
    MirrorVerifierContract,
    r#"[
        function postMirror(address creator, bytes32 commitment, uint64 blockHeight, uint64 timestampMs, bytes signature)
        function mirrors(bytes32) view returns (address creator, bytes32 commitment, uint64 blockHeight, uint64 timestampMs)
    ]"#
);

#[derive(Serialize)]
struct LivingInscription<'a> {
    protocol: &'a str,
    content_type: &'a str,
    body: &'a str,
    metadata: &'a str,
}

const MIRROR_VERIFIER_SOURCE: &str = include_str!("../MirrorVerifier.sol");

#[tokio::main]
async fn main() -> AnyResult<()> {
    let inscription = LivingInscription {
        protocol: "ord-mirror-v0",
        content_type: "application/json",
        body: "{\"artifact\":\"Hello from Bitcoin!\"}",
        metadata: "{\"creator\":\"mirror-bridge\",\"network\":\"signet\"}",
    };

    let inscription_bytes = serde_json::to_vec(&inscription)?;
    let mut hasher = Hasher::new();
    hasher.update(&inscription_bytes);
    let commitment = hasher.finalize();
    let mut commitment_bytes = [0u8; 32];
    commitment_bytes.copy_from_slice(commitment.as_bytes());

    println!("Living inscription JSON: {}", String::from_utf8_lossy(&inscription_bytes));
    println!("Commitment (blake3): 0x{}", hex::encode(commitment_bytes));

    let anvil = Anvil::new().spawn();

    let wallet: Wallet<SigningKey> = anvil.keys()[0].clone().with_chain_id(anvil.chain_id());
    let provider = Provider::<Http>::try_from(anvil.endpoint())?.interval(std::time::Duration::from_millis(10));
    let client: Client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    let compile_output = compile_contract()?;
    let (abi, bytecode) = extract_contract_artifacts(&compile_output)?;

    let factory = ContractFactory::new(abi, bytecode, client.clone());
    let deployed = factory.deploy(())?.send().await?;
    let contract = MirrorVerifierContract::new(deployed.address(), client.clone());

    let creator = wallet.address();
    let block_height = 820_000u64;
    let timestamp_ms = 1_701_234_567_890u64;

    let packed = encode_packed(&[
        Token::Address(creator),
        Token::FixedBytes(commitment_bytes.to_vec()),
        Token::Uint(U256::from(block_height)),
        Token::Uint(U256::from(timestamp_ms)),
    ])?;
    let digest = H256::from(keccak256(packed));

    let signature = wallet.sign_message(digest).await?.to_vec();

    println!("Posting mirror from {}", creator);
    contract
        .post_mirror(
            creator,
            H256::from(commitment_bytes),
            block_height,
            timestamp_ms,
            Bytes::from(signature.clone()),
        )
        .send()
        .await?
        .await?;

    let record = contract.mirrors(H256::from(commitment_bytes)).call().await?;

    println!(
        "Stored mirror record: creator={}, block={}, timestamp={}",
        record.creator, record.block_height, record.timestamp_ms
    );

    assert_eq!(record.creator, creator);
    assert_eq!(record.commitment, H256::from(commitment_bytes));
    assert_eq!(record.block_height, block_height);
    assert_eq!(record.timestamp_ms, timestamp_ms);

    println!("Signature used: 0x{}", hex::encode(signature));

    Ok(())
}

fn compile_contract() -> AnyResult<ProjectCompileOutput> {
    let tmp_dir = tempdir()?;
    let sources = tmp_dir.path().join("contracts");
    std::fs::create_dir_all(&sources)?;
    std::fs::write(sources.join("MirrorVerifier.sol"), MIRROR_VERIFIER_SOURCE)?;

    let paths = ProjectPathsConfig::builder()
        .root(tmp_dir.path())
        .sources(sources)
        .build()?;

    let solc = Solc::find_or_install_svm_version("0.8.24")?;
    let project = Project::builder().paths(paths).solc(solc).build()?;
    let output = project.compile()?;
    output.ensure_no_errors()?;

    Ok(output)
}

fn extract_contract_artifacts(output: &ProjectCompileOutput) -> AnyResult<(ethers::abi::Abi, Bytes)> {
    let contract = output
        .find("contracts/MirrorVerifier.sol", "MirrorVerifier")
        .ok_or("MirrorVerifier artifact not found")?;

    let abi = contract.abi.clone().ok_or("Missing ABI")?.into();
    let bytecode = contract
        .bytecode
        .clone()
        .ok_or("Missing bytecode")?
        .object
        .into_bytes()
        .ok_or("Bytecode not available")?;

    Ok((abi, Bytes::from(bytecode)))
}
