use blake3::hash;
use chrono::Utc;
use serde_json::{Map, Value, json};

use crate::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};

/// Fuse two living inscriptions together by blending their traits.
pub fn fuse_with_traits(a: &LivingInscription, b: &LivingInscription) -> LivingInscription {
  let mut combined_traits: Map<String, Value> = Map::new();

  if let Some(traits) = a.traits() {
    for (key, value) in traits {
      combined_traits.insert(key.clone(), value.clone());
    }
  }

  if let Some(traits) = b.traits() {
    for (key, value) in traits {
      match combined_traits.get_mut(key) {
        Some(existing) if existing.is_number() && value.is_number() => {
          if let (Some(lhs), Some(rhs)) = (existing.as_f64(), value.as_f64()) {
            *existing = json!((lhs + rhs) / 2.0);
          }
        }
        Some(_) => {
          // Keep the existing textual trait from the first parent.
        }
        None => {
          combined_traits.insert(key.clone(), value.clone());
        }
      }
    }
  }

  let metadata = json!({
      "traits": Value::Object(combined_traits.clone()),
      "parents": [a.commitment(), b.commitment()],
  });

  let fusion_seed = hash(metadata.to_string().as_bytes());

  let fused_core = InscriptionCore {
    version: 1,
    parent_hash: Some(format!("{}+{}", a.commitment(), b.commitment())),
    creator: format!("fusion:{}:{}", a.core.creator, b.core.creator),
    timestamp: Utc::now(),
    content_uri: format!("ipfs://fusion/{}", fusion_seed.to_hex()),
    metadata,
  };

  let fused_state = InscriptionState {
    block_height: a.state.block_height.max(b.state.block_height) + 1,
    external_entropy: Some(fusion_seed.to_hex().to_string()),
    mood: Some("harmonized".to_string()),
    mirror_hash: Some(hash(fused_core.content_uri.as_bytes()).to_hex().to_string()),
  };

  LivingInscription {
    core: fused_core,
    state: fused_state,
    signature: "0xfused".into(),
  }
}
