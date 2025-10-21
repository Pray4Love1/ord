```rust
use blake3::Hasher;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Core immutable attributes of a living inscription.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InscriptionCore {
  pub version: u32,
  pub parent_hash: Option<String>,
  pub creator: String,
  pub timestamp: DateTime<Utc>,
  pub content_uri: String,
  pub metadata: Value,
}

/// Mutable on-chain and emotional state of an inscription.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InscriptionState {
  pub block_height: u64,
  pub external_entropy: Option<String>,
  pub mood: Option<String>,
  pub mirror_hash: Option<String>,
}

/// The complete living inscription entity with a verifiable signature.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LivingInscription {
  pub core: InscriptionCore,
  pub state: InscriptionState,
  pub signature: String,
}

impl LivingInscription {
  /// Generate a deterministic commitment hash for this living inscription.
  pub fn commitment(&self) -> String {
    let core = serde_json::to_vec(&self.core).expect("core serialization");
    let state = serde_json::to_vec(&self.state).expect("state serialization");

    let mut hasher = Hasher::new();
    hasher.update(&core);
    hasher.update(&state);
    hasher.update(self.signature.as_bytes());

    hasher.finalize().to_hex().to_string()
  }

  /// Derive a readable identifier for display or indexing.
  pub fn id(&self) -> String {
    format!("{}:{}", self.core.creator, self.core.timestamp.timestamp())
  }
}
```
