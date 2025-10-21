//! Basic deterministic genetics utilities for living inscriptions.

use std::collections::BTreeMap;

use crate::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};
use serde_json::{Map, Number, Value, json};

/// Combine two inscriptions into a new child inscription with blended traits.
pub fn fuse_with_traits(a: &LivingInscription, b: &LivingInscription) -> LivingInscription {
  let mut traits: BTreeMap<String, f64> = BTreeMap::new();

  merge_traits_into(&mut traits, a);
  merge_traits_into(&mut traits, b);

  if !traits.contains_key("energy") {
    traits.insert("energy".into(), 1.0);
  }

  let mut traits_json = Map::new();
  for (name, value) in traits {
    if let Some(number) = Number::from_f64(value) {
      traits_json.insert(name, Value::Number(number));
    }
  }

  let child_name = format!("{}-{}", a.core.name, b.core.name);
  let mut metadata = Map::new();
  metadata.insert("name".into(), json!(child_name.clone()));
  metadata.insert("traits".into(), Value::Object(traits_json));
  metadata.insert(
    "lineage".into(),
    json!({
      "parents": [a.commitment(), b.commitment()],
    }),
  );

  let commitment_seed = format!("fusion:{}:{}", a.commitment(), b.commitment());
  let commitment = LivingInscription::commitment_from_seed(commitment_seed.as_bytes());
  let core = InscriptionCore::new(child_name, Value::Object(metadata));

  LivingInscription::from_parts(commitment, core, InscriptionState::default())
}

fn merge_traits_into(target: &mut BTreeMap<String, f64>, source: &LivingInscription) {
  if let Some(traits) = source
    .core
    .metadata
    .get("traits")
    .and_then(Value::as_object)
  {
    for (name, value) in traits {
      if let Some(v) = value.as_f64() {
        target
          .entry(name.clone())
          .and_modify(|existing| {
            *existing = (*existing + v) / 2.0;
          })
          .or_insert(v);
      }
    }
  }
//! Ordinal Genetics v0.1
//! Trait inheritance and mutation for LivingInscriptions

use crate::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};
use chrono::{Duration, Utc};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde_json::{Map, Value, json};

/// Fuse two parents into a child with inheritable and mutable traits.
pub fn fuse_with_traits(a: &LivingInscription, b: &LivingInscription) -> LivingInscription {
  let seed_src = format!("{}{}", a.commitment(), b.commitment());
  let seed_hash = blake3::hash(seed_src.as_bytes());
  let mut rng = ChaCha20Rng::from_seed(*seed_hash.as_bytes());

  let traits_a = a
    .core
    .metadata
    .get("traits")
    .cloned()
    .unwrap_or_else(|| json!({}));
  let traits_b = b
    .core
    .metadata
    .get("traits")
    .cloned()
    .unwrap_or_else(|| json!({}));
  let merged_traits = inherit_traits(traits_a, traits_b, &mut rng);

  let seed_bytes = seed_hash.as_bytes();
  let lineage_hash = seed_hash.to_hex().to_string();
  let base_timestamp = if a.core.timestamp >= b.core.timestamp {
    a.core.timestamp
  } else {
    b.core.timestamp
  };
  let offset_seconds = (u16::from_le_bytes([seed_bytes[0], seed_bytes[1]]) % 3_600) as i64;
  let timestamp = base_timestamp + Duration::seconds(offset_seconds);
  let core = InscriptionCore {
    version: 1,
    parent_hash: Some(lineage_hash.clone()),
    creator: format!("fusion({}+{})", a.core.creator, b.core.creator),
    timestamp,
    content_uri: format!("ipfs://Fusion{}", lineage_hash),
    metadata: json!({
      "traits": merged_traits,
      "mutation_seed": lineage_hash,
      "parents": [a.commitment(), b.commitment()],
    }),
  };

  let state = InscriptionState {
    block_height: std::cmp::max(a.state.block_height, b.state.block_height) + 1,
    external_entropy: Some(lineage_hash.clone()),
    mood: Some("metamorphic".into()),
    mirror_hash: None,
  };

  LivingInscription {
    core,
    state,
    signature: "0xgenesisfusion".into(),
  }
}

/// Blend numeric and categorical traits, with slight mutation.
fn inherit_traits(a: Value, b: Value, rng: &mut ChaCha20Rng) -> Value {
  let map_a = a.as_object().cloned().unwrap_or_default();
  let map_b = b.as_object().cloned().unwrap_or_default();

  let keys = map_a
    .keys()
    .chain(map_b.keys())
    .cloned()
    .collect::<std::collections::BTreeSet<_>>();

  let mut child = Map::new();

  for key in keys {
    match (map_a.get(&key), map_b.get(&key)) {
      (Some(Value::Number(x)), Some(Value::Number(y))) => {
        let avg = (x.as_f64().unwrap_or(0.0) + y.as_f64().unwrap_or(0.0)) / 2.0;
        let delta = rng.gen_range(-0.05..0.05);
        child.insert(key, json!(avg * (1.0 + delta)));
      }
      (Some(Value::String(xs)), Some(Value::String(ys))) => {
        let inherit = if rng.gen_bool(0.5) { xs } else { ys };
        let value = if rng.gen_bool(0.05) {
          format!("{}*", inherit)
        } else {
          inherit.clone()
        };
        child.insert(key, json!(value));
      }
      (Some(v), None) | (None, Some(v)) => {
        child.insert(key, v.clone());
      }
      _ => {}
    }
  }

  Value::Object(child)
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn inherits_numeric_and_string_traits() {
    let traits = inherit_traits(
      json!({"energy": 1.0, "color": "red"}),
      json!({"energy": 0.6, "color": "blue"}),
      &mut ChaCha20Rng::from_seed([1; 32]),
    );

    assert!(traits.get("energy").is_some());
    assert!(traits.get("color").is_some());
  }
}
