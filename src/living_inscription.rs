//! Types describing living inscriptions used by both viewer and protocol layers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Minimal metadata for viewer and indexing.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InscriptionCore {
    /// URI pointing at the inscription's current content.
    pub content_uri: String,
    /// Parent commitment hash linking this inscription to its lineage.
    pub parent_hash: Option<String>,
}

impl InscriptionCore {
    /// Construct a new [`InscriptionCore`] with the provided content URI and optional parent hash.
    pub fn new(content_uri: impl Into<String>, parent_hash: Option<impl Into<String>>) -> Self {
        Self {
            content_uri: content_uri.into(),
            parent_hash: parent_hash.map(Into::into),
        }
    }
}

/// Full metadata with creator, version, and timestamp for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscriptionMeta {
    pub version: u32,
    pub creator: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Live state describing on-chain and external context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscriptionState {
    pub block_height: u64,
    pub external_entropy: Option<String>,
    pub mood: Option<String>,
    pub mirror_hash: Option<String>,
}

/// A signed living inscription tying everything together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingInscription {
    pub core: InscriptionCore,
    pub meta: InscriptionMeta,
    pub state: InscriptionState,
    pub signature: String,
}

impl LivingInscription {
    pub fn id(&self) -> String {
        format!("{}:{}", self.meta.creator, self.meta.timestamp.timestamp())
    }
}