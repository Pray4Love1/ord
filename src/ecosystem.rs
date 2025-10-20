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
      evolved_b.core.metadata["traits"]["energy"] = serde_json::json!(
        0.8
          * evolved_b.core.metadata["traits"]["energy"]
            .as_f64()
            .unwrap_or(1.0)
      );
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
