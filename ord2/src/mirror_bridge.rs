use crate::LivingInscription;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use sled::IVec;
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;

/// Stored representation used by the viewer API and CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorRecord {
  pub commitment: String,
  pub block_height: u64,
  pub timestamp: chrono::DateTime<chrono::Utc>,
  pub parent_hash: Option<String>,
  pub entropy: f64,
  pub metadata: serde_json::Value,
}

impl From<LivingInscription> for MirrorRecord {
  fn from(value: LivingInscription) -> Self {
    Self {
      metadata: serde_json::Value::Object(
        value
          .metadata
          .into_iter()
          .collect::<Map<String, serde_json::Value>>(),
      ),
      commitment: value.commitment,
      block_height: value.block_height,
      timestamp: value.timestamp,
      parent_hash: value.parent_hash,
      entropy: value.entropy,
    }
  }
}

impl MirrorRecord {
  fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
    Ok(serde_json::to_vec(self)?)
  }

  fn from_bytes(bytes: &IVec) -> anyhow::Result<Self> {
    Ok(serde_json::from_slice(bytes)?)
  }
}

/// Errors produced by the mirror bridge.
#[derive(Debug, Error)]
pub enum BridgeError {
  #[error("database error: {0}")]
  Database(#[from] sled::Error),
  #[error("serialization error: {0}")]
  Serialization(#[from] serde_json::Error),
}

/// Lightweight storage bridge shared by the watcher, API, and viewer.
#[derive(Clone)]
pub struct MirrorBridge {
  db: Arc<sled::Db>,
}

impl MirrorBridge {
  pub fn new(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
    let path = path.into();
    let db = sled::open(path)?;
    Ok(Self { db: Arc::new(db) })
  }

  fn key(commitment: &str) -> Vec<u8> {
    commitment.as_bytes().to_vec()
  }

  pub fn store(&self, inscription: &LivingInscription) -> Result<MirrorRecord, BridgeError> {
    let record: MirrorRecord = inscription.clone().into();
    let key = Self::key(&record.commitment);
    let value = record.to_bytes().map_err(BridgeError::Serialization)?;
    self.db.insert(key, value)?;
    self.db.flush()?;
    Ok(record)
  }

  pub fn get(&self, commitment: &str) -> anyhow::Result<Option<MirrorRecord>> {
    let Some(value) = self.db.get(Self::key(commitment))? else {
      return Ok(None);
    };
    MirrorRecord::from_bytes(&value)
      .map(Some)
      .map_err(Into::into)
  }

  pub fn list(&self) -> anyhow::Result<Vec<MirrorRecord>> {
    let mut records = Vec::new();
    for result in self.db.iter() {
      let (_, value) = result?;
      records.push(MirrorRecord::from_bytes(&value)?);
    }
    records.sort_by_key(|record| record.block_height);
    Ok(records)
  }

  pub fn delete(&self, commitment: &str) -> anyhow::Result<bool> {
    Ok(self.db.remove(Self::key(commitment))?.is_some())
  }

  pub fn path(&self) -> anyhow::Result<PathBuf> {
    Ok(self.db.path().to_path_buf())
  }
}

impl Default for MirrorBridge {
  fn default() -> Self {
    let path = default_db_path();
    Self::new(path).expect("failed to open default mirror database")
  }
}

pub fn default_db_path() -> PathBuf {
  std::env::var("MIRROR_DB")
    .map(PathBuf::from)
    .unwrap_or_else(|_| PathBuf::from("./mirror_db"))
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  #[test]
  fn store_and_fetch_roundtrip() {
    let dir = tempdir().unwrap();
    let bridge = MirrorBridge::new(dir.path().join("db")).unwrap();

    let inscription = LivingInscription::simulated(42);
    let stored = bridge.store(&inscription).unwrap();
    assert_eq!(stored.commitment, inscription.commitment);

    let fetched = bridge.get(&stored.commitment).unwrap().unwrap();
    assert_eq!(fetched.commitment, stored.commitment);
  }
}
