use chrono::Utc;
use serde::Serialize;
use serde_json::{json, Value};

use crate::living_inscription::LivingInscription;

#[derive(Debug, Serialize)]
pub struct MirrorProof {
    pub network: String,
    pub commitment: String,
    pub block_height: u64,
    pub timestamp_ms: i64,
    pub creator: String,
    pub metadata: Value,
}

pub fn build_proof(inscription: &LivingInscription, network: &str) -> MirrorProof {
    let commitment = inscription.commitment();
    let timestamp_ms = Utc::now().timestamp_millis();

    MirrorProof {
        network: network.to_string(),
        commitment,
        block_height: inscription.state.block_height,
        timestamp_ms,
        creator: inscription.core.creator.clone(),
        metadata: json!({
            "external_entropy": inscription.state.external_entropy.clone(),
            "mood": inscription.state.mood.clone(),
            "mirror_hash": inscription.state.mirror_hash.clone(),
            "core_metadata": inscription.core.metadata.clone(),
        }),
    }
}
