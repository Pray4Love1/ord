use blake3::Hasher;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

/// Core inscription data describing provenance and metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Immutable core data for a living inscription.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// Runtime state that drives Living Inscriptions.
#[derive(Clone, Debug, Serialize, Deserialize)]
  pub metadata: serde_json::Value,
}

/// Mutable state for a living inscription.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// Complete inscription containing core data, state, and signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
/// A living inscription containing immutable and mutable data with a signature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// The complete living inscription entity with a verifiable signature.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LivingInscription {
  pub core: InscriptionCore,
  pub state: InscriptionState,
  pub signature: String,
}

impl LivingInscription {
  /// Deterministic commitment derived from the full inscription contents.
  pub fn commitment(&self) -> String {
    let payload = json!({
        "core": {
            "version": self.core.version,
            "parent_hash": self.core.parent_hash,
            "creator": self.core.creator,
            "timestamp": self.core.timestamp.to_rfc3339(),
            "content_uri": self.core.content_uri,
            "metadata": self.core.metadata,
        },
        "state": {
            "block_height": self.state.block_height,
            "external_entropy": self.state.external_entropy,
            "mood": self.state.mood,
            "mirror_hash": self.state.mirror_hash,
        },
        "signature": self.signature,
    });

    let mut hasher = Hasher::new();
    hasher.update(payload.to_string().as_bytes());
    hasher.finalize().to_hex().to_string()
  }

  /// Convenience accessor for metadata traits.
  pub fn traits(&self) -> Option<&Map<String, Value>> {
    self
      .core
      .metadata
      .get("traits")
      .and_then(|value| value.as_object())
  }
}

/// Build a deterministic sample inscription for testing purposes.
pub fn sample_inscription(name: &str) -> LivingInscription {
  let seed = blake3::hash(name.as_bytes());
  let energy = 0.75 + (seed.as_bytes()[0] as f64 / 255.0) * 0.1;
  let color = if seed.as_bytes()[1] % 2 == 0 {
    "red"
  } else {
    "blue"
  };

  let metadata = json!({
      "name": name,
      "traits": {
          "energy": energy,
          "color": color,
      },
  });

  LivingInscription {
    core: InscriptionCore {
      version: 1,
      parent_hash: None,
      creator: format!("creator-{}", name),
      timestamp: Utc::now(),
      content_uri: format!("ipfs://seed/{}", seed.to_hex()),
      metadata,
    },
    state: InscriptionState {
      block_height: 0,
      external_entropy: None,
      mood: Some("curious".to_string()),
      mirror_hash: None,
    },
    signature: format!("0x{}_sig", name.to_lowercase()),
  }
}
  /// Calculate the deterministic commitment hash for the inscription.
  pub fn commitment(&self) -> String {
    #[derive(Serialize)]
    struct CommitmentInput<'a> {
      core: &'a InscriptionCore,
      state: &'a InscriptionState,
    }

    let encoded = serde_json::to_vec(&CommitmentInput {
      core: &self.core,
      state: &self.state,
    })
    .expect("LivingInscription should serialize to JSON");
    blake3::hash(&encoded).to_hex().to_string()
  }
}
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
