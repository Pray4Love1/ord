use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscriptionCore {
  pub version: u32,
  pub parent_hash: Option<String>,
  pub creator: String,
  pub timestamp: DateTime<Utc>,
  pub content_uri: String,
  pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscriptionState {
  pub block_height: u64,
  pub external_entropy: Option<String>,
  pub mood: Option<String>,
  pub mirror_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingInscription {
  pub core: InscriptionCore,
  pub state: InscriptionState,
  pub signature: String,
}

impl LivingInscription {
  pub fn id(&self) -> String {
    format!("{}:{}", self.core.creator, self.core.timestamp.timestamp())
  }
}
