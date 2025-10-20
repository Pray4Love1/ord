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
}
