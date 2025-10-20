use crate::LivingInscription;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::{
  fs,
  path::PathBuf,
  sync::{Arc, Mutex},
};
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

/// Errors produced by the mirror bridge.
#[derive(Debug, Error)]
pub enum BridgeError {
  #[error("database error: {0}")]
  Database(#[from] rusqlite::Error),
  #[error("serialization error: {0}")]
  Serialization(#[from] serde_json::Error),
}

/// Lightweight storage bridge shared by the watcher, API, and viewer.
#[derive(Clone)]
pub struct MirrorBridge {
  conn: Arc<Mutex<Connection>>,
  path: Arc<PathBuf>,
}

impl MirrorBridge {
  pub fn new(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
    let path = path.into();
    if let Some(parent) = path.parent() {
      if !parent.as_os_str().is_empty() {
        fs::create_dir_all(parent)?;
      }
    }

    let conn = Connection::open(&path)?;
    Self::init_schema(&conn)?;

    Ok(Self {
      conn: Arc::new(Mutex::new(conn)),
      path: Arc::new(path),
    })
  }

  pub fn store(&self, inscription: &LivingInscription) -> Result<MirrorRecord, BridgeError> {
    let record: MirrorRecord = inscription.clone().into();
    let json = serde_json::to_string(&record).map_err(BridgeError::Serialization)?;

    let conn = self.conn.lock().expect("mirror bridge mutex poisoned");
    conn.execute(
      "INSERT INTO mirror_records (commitment, block_height, record_json)
       VALUES (?1, ?2, ?3)
       ON CONFLICT(commitment) DO UPDATE SET
         block_height = excluded.block_height,
         record_json = excluded.record_json",
      params![&record.commitment, record.block_height as i64, json],
    )?;
    Ok(record)
  }

  pub fn get(&self, commitment: &str) -> anyhow::Result<Option<MirrorRecord>> {
    let conn = self.conn.lock().expect("mirror bridge mutex poisoned");
    let record: Option<String> = conn
      .query_row(
        "SELECT record_json FROM mirror_records WHERE commitment = ?1",
        params![commitment],
        |row| row.get(0),
      )
      .optional()?;

    record
      .map(|json| serde_json::from_str(&json).map_err(anyhow::Error::from))
      .transpose()
  }

  pub fn list(&self) -> anyhow::Result<Vec<MirrorRecord>> {
    let conn = self.conn.lock().expect("mirror bridge mutex poisoned");
    let mut stmt = conn.prepare(
      "SELECT record_json FROM mirror_records ORDER BY block_height ASC, commitment ASC",
    )?;
    let records_iter = stmt.query_map([], |row| {
      let json: String = row.get(0)?;
      serde_json::from_str::<MirrorRecord>(&json).map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(
          json.len(),
          rusqlite::types::Type::Text,
          Box::new(err),
        )
      })
    })?;

    let mut records = Vec::new();
    for record in records_iter {
      records.push(record?);
    }
    Ok(records)
  }

  pub fn delete(&self, commitment: &str) -> anyhow::Result<bool> {
    let conn = self.conn.lock().expect("mirror bridge mutex poisoned");
    let rows = conn.execute(
      "DELETE FROM mirror_records WHERE commitment = ?1",
      params![commitment],
    )?;
    Ok(rows > 0)
  }

  pub fn path(&self) -> anyhow::Result<PathBuf> {
    Ok(self.path.as_ref().clone())
  }

  fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
      "CREATE TABLE IF NOT EXISTS mirror_records (
         commitment TEXT PRIMARY KEY,
         block_height INTEGER NOT NULL,
         record_json TEXT NOT NULL
       )",
      [],
    )?;
    Ok(())
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
