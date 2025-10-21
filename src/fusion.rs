//! Ordinal Fusion v0.1
//! Combines two LivingInscriptions to create a child with merged metadata.

use crate::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};
use chrono::Utc;
use serde_json::Value;

/// Fuse two inscriptions into a new one
pub fn fuse(a: &LivingInscription, b: &LivingInscription) -> LivingInscription {
  // Combine commitments to form lineage hash
  let lineage_input = format!("{}{}", a.commitment(), b.commitment());
  let lineage_hash = blake3::hash(lineage_input.as_bytes()).to_hex().to_string();

  // Merge metadata (preferring newer keys from b)
  let merged_meta = merge_json(a.core.metadata.clone(), b.core.metadata.clone());

  let core = InscriptionCore {
    version: 1,
    parent_hash: Some(lineage_hash.clone()),
    creator: format!("fused:{}+{}", a.core.creator, b.core.creator),
    timestamp: Utc::now(),
    content_uri: format!("ipfs://Fusion{}", lineage_hash),
    metadata: merged_meta,
  };

  let state = InscriptionState {
    block_height: std::cmp::max(a.state.block_height, b.state.block_height) + 1,
    external_entropy: Some(lineage_hash.clone()),
    mood: Some("emergent".into()),
    mirror_hash: None,
  };

  LivingInscription {
    core,
    state,
    signature: "0xfusion".into(), // In production you’d sign this with your fusion key
  }
}

/// Merge two JSON metadata objects recursively
fn merge_json(mut a: Value, b: Value) -> Value {
  match (a, b) {
    (Value::Object(mut a_map), Value::Object(b_map)) => {
      for (k, v) in b_map {
        match a_map.remove(&k) {
          Some(existing) => {
            let merged_value = merge_json(existing, v);
            a_map.insert(k, merged_value);
          }
          None => {
            a_map.insert(k, v);
          }
        }
      }
      Value::Object(a_map)
    }
    (_, b_non_obj) => b_non_obj,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn merge_json_prefers_newer_values() {
    let a = json!({
      "name": "Alpha",
      "stats": {
        "power": 1,
        "speed": 2
      }
    });
    let b = json!({
      "stats": {
        "speed": 3,
        "focus": 4
      },
      "rarity": "legendary"
    });

    let merged = merge_json(a, b);

    assert_eq!(merged["name"], json!("Alpha"));
    assert_eq!(merged["stats"]["power"], json!(1));
    assert_eq!(merged["stats"]["speed"], json!(3));
    assert_eq!(merged["stats"]["focus"], json!(4));
    assert_eq!(merged["rarity"], json!("legendary"));
  }
}
