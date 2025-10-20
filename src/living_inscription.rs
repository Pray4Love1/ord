//! Types describing living inscriptions used by the viewer.

use serde::{Deserialize, Serialize};

/// Core living inscription metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InscriptionCore {
  /// URI pointing at the inscription's current content.
  pub content_uri: String,
  /// Parent commitment hash that links this inscription to its lineage.
  pub parent_hash: Option<String>,
}

impl InscriptionCore {
  /// Construct a new [`InscriptionCore`] with the provided content URI and optional parent hash.
  pub fn new(content_uri: impl Into<String>, parent_hash: Option<impl Into<String>>) -> Self {
    Self {
      content_uri: content_uri.into(),
      parent_hash: parent_hash.map(Into::into),
    }
  }
}
