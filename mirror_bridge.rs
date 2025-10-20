//! MirrorBridge v0.1 — minimal cross-chain proof emitter for Living Inscription
//! Rust 2024 compatible

use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use ethers::abi::{self, Token};
use ethers::types::Address;
use ethers::utils::keccak256;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::living_inscription::LivingInscription;

const PROOF_TYPEHASH: &[u8] = b"MirrorProof(bytes32 inscriptionCommitment,address creator,uint64 blockHeight,string mirrorChain,uint64 timestampMs)";
const DOMAIN_TYPEHASH: &[u8] = b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
const NAME_HASH: &[u8] = b"MirrorVerifier";
const VERSION_HASH: &[u8] = b"1";

/// Minimal payload transmitted to EVM mirror chains.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MirrorProof {
    pub inscription_commitment: String, // blake3 hash hex string
    pub creator: String,                // hex-encoded EVM address
    pub block_height: u64,
    pub mirror_chain: String,
    pub signature: String, // hex-encoded secp256k1 signature
    pub timestamp_ms: u64,
}

impl MirrorProof {
    fn commitment_bytes(&self) -> Result<[u8; 32]> {
        let commitment = self.inscription_commitment.trim_start_matches("0x");
        let bytes = hex::decode(commitment)
            .with_context(|| format!("invalid commitment hex: {}", self.inscription_commitment))?;
        if bytes.len() != 32 {
            bail!("commitment must be 32 bytes, got {}", bytes.len());
        }
        let mut output = [0u8; 32];
        output.copy_from_slice(&bytes);
        Ok(output)
    }

    fn creator_address(&self) -> Result<Address> {
        Address::from_str(self.creator.as_str()).with_context(|| {
            format!(
                "creator must be a checksummed 0x-prefixed address, got {}",
                self.creator
            )
        })
    }

    fn signature_bytes(&self) -> Result<Vec<u8>> {
        let signature = self.signature.trim_start_matches("0x");
        hex::decode(signature)
            .with_context(|| format!("invalid signature hex: {}", self.signature))
    }

    fn struct_hash(&self) -> Result<[u8; 32]> {
        let type_hash = keccak256(PROOF_TYPEHASH);
        let creator = self.creator_address()?;
        let commitment = self.commitment_bytes()?;
        let chain_hash = keccak256(self.mirror_chain.as_bytes());

        let encoded = abi::encode(&[
            Token::FixedBytes(type_hash.to_vec()),
            Token::FixedBytes(commitment.to_vec()),
            Token::Address(creator),
            Token::Uint(self.block_height.into()),
            Token::FixedBytes(chain_hash.to_vec()),
            Token::Uint(self.timestamp_ms.into()),
        ]);

        Ok(keccak256(&encoded))
    }

    fn domain_separator(&self, chain_id: u64, verifying_contract: Address) -> [u8; 32] {
        let type_hash = keccak256(DOMAIN_TYPEHASH);
        let name_hash = keccak256(NAME_HASH);
        let version_hash = keccak256(VERSION_HASH);

        let encoded = abi::encode(&[
            Token::FixedBytes(type_hash.to_vec()),
            Token::FixedBytes(name_hash.to_vec()),
            Token::FixedBytes(version_hash.to_vec()),
            Token::Uint(chain_id.into()),
            Token::Address(verifying_contract),
        ]);

        keccak256(&encoded)
    }

    /// Returns the EIP-712 digest expected by the MirrorVerifier contract.
    pub fn typed_data_digest(&self, chain_id: u64, verifying_contract: Address) -> Result<[u8; 32]> {
        let domain = self.domain_separator(chain_id, verifying_contract);
        let struct_hash = self.struct_hash()?;

        let mut digest_input = Vec::with_capacity(66);
        digest_input.extend_from_slice(b"\x19\x01");
        digest_input.extend_from_slice(&domain);
        digest_input.extend_from_slice(&struct_hash);
        Ok(keccak256(&digest_input))
    }

    fn solidity_tuple(&self) -> Result<Vec<Token>> {
        let commitment = self.commitment_bytes()?;
        let creator = self.creator_address()?;
        let signature = self.signature_bytes()?;

        Ok(vec![
            Token::FixedBytes(commitment.to_vec()),
            Token::Address(creator),
            Token::Uint(self.block_height.into()),
            Token::String(self.mirror_chain.clone()),
            Token::Bytes(signature),
            Token::Uint(self.timestamp_ms.into()),
        ])
    }
}

/// Builds a new proof from a LivingInscription object.
pub fn build_proof(ins: &LivingInscription, mirror_chain: &str) -> Result<MirrorProof> {
    Ok(MirrorProof {
        inscription_commitment: ins.commitment(),
        creator: ins.core
            .creator
            .clone(),
        block_height: ins.state.block_height,
        mirror_chain: mirror_chain.to_string(),
        signature: ins.signature.clone(),
        timestamp_ms: Utc::now().timestamp_millis() as u64,
    })
}

/// Posts the proof to a minimal EVM mirror endpoint (HTTP JSON-RPC).
pub async fn post_proof(
    proof: &MirrorProof,
    rpc_url: &str,
    from: Address,
    mirror_contract: Address,
) -> Result<()> {
    let proof_tokens = proof.solidity_tuple()?;
    let call_data = encode_submit_proof(&proof_tokens)?;

    let tx = serde_json::json!({
        "from": format!("{:#x}", from),
        "to": format!("{:#x}", mirror_contract),
        "data": format!("0x{}", hex::encode(call_data)),
        "value": "0x0"
    });

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_sendTransaction",
        "params": [tx],
        "id": 1
    });

    let client = Client::new();
    let resp = client.post(rpc_url).json(&req).send().await?;
    let text = resp.text().await?;
    println!("Mirror response: {text}");
    Ok(())
}

fn encode_submit_proof(tokens: &[Token]) -> Result<Vec<u8>> {
    if tokens.len() != 6 {
        return Err(anyhow!("expected six tuple values"));
    }

    let selector = &keccak256(b"submitProof((bytes32,address,uint64,string,bytes,uint64))")[..4];
    let encoded = abi::encode(&[Token::Tuple(tokens.to_vec())]);

    let mut out = Vec::with_capacity(4 + encoded.len());
    out.extend_from_slice(selector);
    out.extend_from_slice(&encoded);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::Address;

    #[test]
    fn digest_is_deterministic() {
        let proof = MirrorProof {
            inscription_commitment: "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
            creator: "0x1111111111111111111111111111111111111111".into(),
            block_height: 1,
            mirror_chain: "sepolia".into(),
            signature: "0x".into(),
            timestamp_ms: 42,
        };

        let contract = Address::from_low_u64_be(7);
        let digest_a = proof
            .typed_data_digest(11155111, contract)
            .expect("digest");
        let digest_b = proof
            .typed_data_digest(11155111, contract)
            .expect("digest");

        assert_eq!(digest_a, digest_b);
    }
}
