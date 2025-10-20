use blake3::Hasher;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

/// Core inscription data describing provenance and metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
pub struct InscriptionState {
  pub block_height: u64,
  pub external_entropy: Option<String>,
  pub mood: Option<String>,
  pub mirror_hash: Option<String>,
}

/// Complete inscription containing core data, state, and signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
