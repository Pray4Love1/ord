use chrono::{DateTime, TimeZone, Utc};
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;

/// Representation of a living inscription record that is mirrored onto the
/// verification network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingInscription {
  pub commitment: String,
  pub block_height: u64,
  pub timestamp: DateTime<Utc>,
  pub parent_hash: Option<String>,
  pub entropy: f64,
  pub metadata: BTreeMap<String, serde_json::Value>,
}

impl LivingInscription {
  /// Create a new living inscription instance.
  pub fn new(
    commitment: impl Into<String>,
    block_height: u64,
    timestamp: DateTime<Utc>,
    parent_hash: Option<String>,
    entropy: f64,
    metadata: BTreeMap<String, serde_json::Value>,
  ) -> Self {
    Self {
      commitment: commitment.into(),
      block_height,
      timestamp,
      parent_hash,
      entropy,
      metadata,
    }
  }

  /// Build a simulated inscription using pseudo-random data.
  pub fn simulated(block_height: u64) -> Self {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    let commitment = format!("0x{}", hex::encode(bytes));

    let parent_hash = if block_height % 3 == 0 {
      let mut parent_bytes = [0u8; 32];
      rng.fill_bytes(&mut parent_bytes);
      Some(format!("0x{}", hex::encode(parent_bytes)))
    } else {
      None
    };

    let entropy = rng.gen_range(0.0..1.0);

    let mut metadata = BTreeMap::new();
    metadata.insert(
      "status".to_string(),
      json!(if entropy > 0.5 { "vital" } else { "dormant" }),
    );
    metadata.insert(
      "observer".to_string(),
      json!(format!("node-{:02}", rng.gen_range(1..=8))),
    );

    let timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()
      + chrono::Duration::seconds(block_height as i64 * 10);

    Self::new(
      commitment,
      block_height,
      timestamp,
      parent_hash,
      entropy,
      metadata,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simulated_values_are_reasonable() {
    let inscription = LivingInscription::simulated(1000);
    assert!(inscription.commitment.starts_with("0x"));
    assert!(inscription.commitment.len() > 2);
    assert!(inscription.entropy >= 0.0 && inscription.entropy <= 1.0);
  }
}
