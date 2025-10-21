//! Ordinal Ecosystem v0.1
//! Simulates interactions among living inscriptions.
//! All activity stays local and deterministic.

use crate::evolution::{EvolutionTrigger, evolve};
use crate::living_inscription::LivingInscription;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

/// Interaction outcomes
#[derive(Debug)]
pub enum Interaction {
  Cooperation,
  Competition,
  Fusion,
  Neutral,
}

/// Decide the type of interaction between two inscriptions.
fn decide_interaction(a: &LivingInscription, b: &LivingInscription) -> Interaction {
  let seed_src = format!("{}{}", a.commitment(), b.commitment());
  let mut rng = ChaCha20Rng::from_seed(*blake3::hash(seed_src.as_bytes()).as_bytes());
  match rng.gen_range(0..100) {
    0..=39 => Interaction::Cooperation,
    40..=69 => Interaction::Competition,
    70..=84 => Interaction::Fusion,
    _ => Interaction::Neutral,
  }
}

/// Apply interaction logic locally.
pub fn simulate_interaction(
  a: &LivingInscription,
  b: &LivingInscription,
) -> Vec<LivingInscription> {
  match decide_interaction(a, b) {
    Interaction::Cooperation => {
      let mut evolved_a = evolve(a, EvolutionTrigger::MoodShift("harmonious".into()));
      let mut evolved_b = evolve(b, EvolutionTrigger::MoodShift("harmonious".into()));
      evolved_a.state.mood = Some("energized".into());
      evolved_b.state.mood = Some("energized".into());
      vec![evolved_a, evolved_b]
    }
    Interaction::Competition => {
      // Slightly dampen the loser’s energy trait.
      let mut evolved_a = evolve(a, EvolutionTrigger::MoodShift("dominant".into()));
      let mut evolved_b = evolve(b, EvolutionTrigger::MoodShift("suppressed".into()));

      // Guard against missing or malformed trait metadata before dampening energy.
      if !evolved_b.core.metadata.is_object() {
        evolved_b.core.metadata = serde_json::json!({});
      }

      if let Some(meta) = evolved_b.core.metadata.as_object_mut() {
        let traits = meta
          .entry("traits".to_string())
          .or_insert_with(|| serde_json::json!({}));

        if !traits.is_object() {
          *traits = serde_json::json!({});
        }

        if let Some(traits_map) = traits.as_object_mut() {
          let current_energy = traits_map
            .get("energy")
            .and_then(|energy| energy.as_f64())
            .unwrap_or(1.0);
          traits_map.insert("energy".to_string(), serde_json::json!(0.8 * current_energy));
        }
      }
      vec![evolved_a, evolved_b]
    }
    Interaction::Fusion => {
      // Use your existing fusion function
      let child = crate::genetics::fuse_with_traits(a, b);
      vec![child]
    }
    Interaction::Neutral => vec![],
  }
}
