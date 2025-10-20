//! Living inscription primitives used by the local evolution lab.

use serde_json::Value;

/// Core inscription data that stays consistent across evolutionary steps.
#[derive(Clone, Debug)]
pub struct InscriptionCore {
  pub name: String,
  pub metadata: Value,
}

impl InscriptionCore {
  /// Construct new core data from a name and metadata blob.
  pub fn new(name: impl Into<String>, metadata: Value) -> Self {
    Self {
      name: name.into(),
      metadata,
    }
  }
}

/// Mutable state for a living inscription.
#[derive(Clone, Debug, Default)]
pub struct InscriptionState {
  pub mood: Option<String>,
}

/// Deterministic, clonable representation of an inscription that can evolve locally.
#[derive(Clone, Debug)]
pub struct LivingInscription {
  commitment: String,
  pub core: InscriptionCore,
  pub state: InscriptionState,
}

impl LivingInscription {
  /// Create a new inscription using the provided name as the commitment seed.
  pub fn new(name: impl Into<String>, metadata: Value) -> Self {
    let name = name.into();
    let commitment = Self::commitment_from_seed(name.as_bytes());
    Self {
      commitment,
      core: InscriptionCore::new(name, metadata),
      state: InscriptionState::default(),
    }
  }

  /// Create an inscription from explicit parts.
  pub fn from_parts(
    commitment: impl Into<String>,
    core: InscriptionCore,
    state: InscriptionState,
  ) -> Self {
    Self {
      commitment: commitment.into(),
      core,
      state,
    }
  }

  /// Deterministically derive a commitment string from the provided seed bytes.
  pub fn commitment_from_seed(seed: impl AsRef<[u8]>) -> String {
    format!("{:x}", blake3::hash(seed.as_ref()))
  }

  /// Get the inscription commitment identifier.
  pub fn commitment(&self) -> &str {
    &self.commitment
  }
}
