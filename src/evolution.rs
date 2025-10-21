//! Simple deterministic evolution helpers for living inscriptions.

use crate::living_inscription::LivingInscription;

/// Events that can trigger an inscription evolution step.
#[derive(Clone, Debug)]
pub enum EvolutionTrigger {
  /// Update the inscription's mood descriptor.
  MoodShift(String),
}

/// Apply an evolutionary step to an inscription.
pub fn evolve(inscription: &LivingInscription, trigger: EvolutionTrigger) -> LivingInscription {
  let mut evolved = inscription.clone();

  match trigger {
    EvolutionTrigger::MoodShift(mood) => {
      evolved.state.mood = Some(mood);
    }
  }

  evolved
//! Ordinal Evolution v0.1
//! Automatic, verifiable state transitions for LivingInscriptions.

use crate::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};
use blake3;
use chrono::Utc;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde_json::json;

/// Evolution trigger types
#[derive(Debug)]
pub enum EvolutionTrigger {
  BlockHeight(u64),
  MoodShift(String),
  EntropyPulse(String),
}

/// Evolve an inscription according to a trigger and return a new state + commitment.
pub fn evolve(ins: &LivingInscription, trigger: EvolutionTrigger) -> LivingInscription {
  let trigger_str = match &trigger {
    EvolutionTrigger::BlockHeight(h) => format!("height:{}", h),
    EvolutionTrigger::MoodShift(m) => format!("mood:{}", m),
    EvolutionTrigger::EntropyPulse(e) => format!("entropy:{}", e),
  };

  // Use trigger + current commitment as seed
  let seed_src = format!("{}:{}", ins.commitment(), trigger_str);
  let seed_hash = blake3::hash(seed_src.as_bytes());
  let mut rng = ChaCha20Rng::from_seed(*seed_hash.as_bytes());

  // Slightly mutate numeric traits
  let mut new_meta = ins.core.metadata.clone();
  if let Some(traits) = new_meta.get_mut("traits") {
    if let Some(map) = traits.as_object_mut() {
      for (_k, v) in map.iter_mut() {
        if let Some(num) = v.as_f64() {
          let delta = rng.gen_range(-0.03..0.03);
          *v = json!(num * (1.0 + delta));
        }
      }
    }
  }

  // Compose the evolved core and state
  let evolved_core = InscriptionCore {
    version: ins.core.version,
    parent_hash: Some(ins.commitment()),
    creator: ins.core.creator.clone(),
    timestamp: Utc::now(),
    content_uri: format!("ipfs://evolve/{}", seed_hash.to_hex()),
    metadata: new_meta,
  };

  let evolved_state = InscriptionState {
    block_height: match &trigger {
      EvolutionTrigger::BlockHeight(h) => *h,
      _ => ins.state.block_height + 1,
    },
    external_entropy: Some(seed_hash.to_hex().to_string()),
    mood: Some(match trigger {
      EvolutionTrigger::MoodShift(m) => m,
      _ => "adaptive".to_string(),
    }),
    mirror_hash: None,
  };

  LivingInscription {
    core: evolved_core,
    state: evolved_state,
    signature: "0xevolved".into(),
  }
}
