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
}
