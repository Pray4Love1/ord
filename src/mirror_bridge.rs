//! Mirror subsystem: shared types and proof submission logic between viewer and verifier.

use anyhow::Result;

use chrono::{DateTime, Utc};

use ethers::types::{Address, H256};

use reqwest::Client;

use serde::{Deserialize, Serialize};

use sha3::{Digest, Sha3_256};

use crate::living_inscription::LivingInscription;

/// Minimal representation of a mirror record returned from the verifier contract.

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MirrorRecord {

    /// Address of the inscription's creator on the target chain.

    pub creator: Address,

    /// Content hash committed to when the mirror was created.

    pub content_hash: H256,

    /// Block height on which the mirror was finalized.

    pub block_height: u64,

    /// Millisecond timestamp recorded by the mirror bridge.

    pub timestamp_ms: u64,

}

impl From<(Address, [u8; 32], u64, u64)> for MirrorRecord {

    fn from(value: (Address, [u8; 32], u64, u64)) -> Self {

        Self {

            creator: value.0,

            content_hash: H256::from_slice(&value.1),

            block_height: value.2,

            timestamp_ms: value.3,

        }

    }

}

/// Proof payload used to attest a [`LivingInscription`] across mirror chains.

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct MirrorProof {

    pub inscription: LivingInscription,

    pub target_chain: String,

    pub proof_hash: String,

    pub issued_at: DateTime<Utc>,

}

/// Construct a new [`MirrorProof`] by hashing creator, signature, and target chain.

pub fn build_proof(inscription: &LivingInscription, target_chain: &str) -> MirrorProof {

    let mut hasher = Sha3_256::new();

    hasher.update(inscription.core.creator.as_bytes());

    hasher.update(inscription.signature.as_bytes());

    hasher.update(target_chain.as_bytes());

    let proof_hash = hasher.finalize();

    MirrorProof {

        inscription: inscription.clone(),

        target_chain: target_chain.to_owned(),

        proof_hash: hex::encode(proof_hash),

        issued_at: Utc::now(),

    }
//! Mirror subsystem: shared types and proof submission logic between viewer and verifier.

use anyhow::Result;

use chrono::{DateTime, Utc};

use ethers::types::{Address, H256};

use reqwest::Client;

use serde::{Deserialize, Serialize};

use sha3::{Digest, Sha3_256};

use crate::living_inscription::LivingInscription;

/// Minimal representation of a mirror record returned from the verifier contract.

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MirrorRecord {

    /// Address of the inscription's creator on the target chain.

    pub creator: Address,

    /// Content hash committed to when the mirror was created.

    pub content_hash: H256,

    /// Block height on which the mirror was finalized.

    pub block_height: u64,

    /// Millisecond timestamp recorded by the mirror bridge.

    pub timestamp_ms: u64,

}

impl From<(Address, [u8; 32], u64, u64)> for MirrorRecord {

    fn from(value: (Address, [u8; 32], u64, u64)) -> Self {

        Self {

            creator: value.0,

            content_hash: H256::from_slice(&value.1),

            block_height: value.2,

            timestamp_ms: value.3,

        }

    }

}

/// Proof payload used to attest a [`LivingInscription`] across mirror chains.

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct MirrorProof {

    pub inscription: LivingInscription,

    pub target_chain: String,

    pub proof_hash: String,

    pub issued_at: DateTime<Utc>,

}

/// Construct a new [`MirrorProof`] by hashing creator, signature, and target chain.

pub fn build_proof(inscription: &LivingInscription, target_chain: &str) -> MirrorProof {

    let mut hasher = Sha3_256::new();

    hasher.update(inscription.core.creator.as_bytes());

    hasher.update(inscription.signature.as_bytes());

    hasher.update(target_chain.as_bytes());

    let proof_hash = hasher.finalize();

    MirrorProof {

        inscription: inscription.clone(),

        target_chain: target_chain.to_owned(),

        proof_hash: hex::encode(proof_hash),

        issued_at: Utc::now(),

    }

}

/// Submit a proof to a remote mirror verifier RPC endpoint.

pub async fn post_proof(

    http: &Client,

    proof: &MirrorProof,

    mirror_rpc: &str,

    verifier: &str,

) -> Result<()> {

    let payload = serde_json::json!({

        "verifier": verifier,

        "target_chain": proof.target_chain,

        "proof_hash": proof.proof_hash,

        "issued_at": proof.issued_at,

        "inscription": proof.inscription,

    });

    let response = http.post(mirror_rpc).json(&payload).send().await?;

    println!(

        "Submitted proof {} to verifier {} (status: {})",

        proof.proof_hash,

        verifier,

        response.status()

    );

    Ok(())

}
}

/// Submit a proof to a remote mirror verifier RPC endpoint.

pub async fn post_proof(

    http: &Client,

    proof: &MirrorProof,

    mirror_rpc: &str,

    verifier: &str,

) -> Result<()> {

    let payload = serde_json::json!({

        "verifier": verifier,

        "target_chain": proof.target_chain,

        "proof_hash": proof.proof_hash,

        "issued_at": proof.issued_at,

        "inscription": proof.inscription,

    });

    let response = http.post(mirror_rpc).json(&payload).send().await?;

    println!(

        "Submitted proof {} to verifier {} (status: {})",

        proof.proof_hash,

        verifier,

        response.status()

    );

    Ok(())

}