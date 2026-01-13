use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::errors::Brc20Error;
use crate::identity::{verify_identity, IdentityProof};
use crate::zk::ZkProof;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vesting {
  pub unlock_block: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferRules {
  pub max_per_tx: Option<u64>,
  pub require_identity: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
  pub name: String,
  pub symbol: String,
  pub decimals: u8,
  pub soulbound: bool,
  pub vesting: BTreeMap<String, Vesting>,
  pub transfer_rules: TransferRules,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenState {
  pub token_id: String,
  pub balances: BTreeMap<String, u64>,
  pub metadata: Metadata,
  pub prev_state_hash: String,
  pub merkle_root: String,
  pub timestamp: i64,
}

pub struct Brc20V2 {
  pub state: TokenState,
}

impl Brc20V2 {
  pub fn new(token_id: &str, metadata: Metadata) -> Self {
    let state = TokenState {
      token_id: token_id.to_string(),
      balances: BTreeMap::new(),
      metadata,
      prev_state_hash: "0".repeat(64),
      merkle_root: String::new(),
      timestamp: Utc::now().timestamp(),
    };
    Self { state }
  }

  fn compute_merkle_root(&self) -> String {
    let mut nodes: Vec<String> = self
      .state
      .balances
      .iter()
      .map(|(key, value)| {
        let entry = format!("{}:{}", key, value);
        let mut hasher = Sha256::new();
        hasher.update(entry.as_bytes());
        hex::encode(hasher.finalize())
      })
      .collect();

    if nodes.is_empty() {
      return String::new();
    }

    while nodes.len() > 1 {
      let mut temp = Vec::new();
      for index in (0..nodes.len()).step_by(2) {
        let combined = if index + 1 < nodes.len() {
          format!("{}{}", nodes[index], nodes[index + 1])
        } else {
          nodes[index].clone()
        };
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        temp.push(hex::encode(hasher.finalize()));
      }
      nodes = temp;
    }
    nodes[0].clone()
  }

  fn compute_state_hash(&self) -> String {
    let json = serde_json::to_string(&self.state).expect("state serialization failed");
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    hex::encode(hasher.finalize())
  }

  pub fn mint(&mut self, address: &str, amount: u64) {
    let balance = self
      .state
      .balances
      .entry(address.to_string())
      .or_insert(0);
    *balance += amount;
    self.update_state();
  }

  pub fn transfer(
    &mut self,
    from: &str,
    to: &str,
    amount: u64,
    current_block: u64,
    identity: Option<&IdentityProof>,
  ) -> Result<ZkProof, Brc20Error> {
    if self.state.metadata.soulbound {
      return Err(Brc20Error::Soulbound);
    }

    if let Some(vesting) = self.state.metadata.vesting.get(from) {
      if current_block < vesting.unlock_block {
        return Err(Brc20Error::VestingLocked(vesting.unlock_block));
      }
    }

    if let Some(max) = self.state.metadata.transfer_rules.max_per_tx {
      if amount > max {
        return Err(Brc20Error::MaxTransferExceeded);
      }
    }

    if self.state.metadata.transfer_rules.require_identity {
      let proof = identity.ok_or(Brc20Error::IdentityFailed)?;
      if !verify_identity(&proof.subject, &proof.commitment) {
        return Err(Brc20Error::IdentityFailed);
      }
    }

    let sender_balance = self
      .state
      .balances
      .get_mut(from)
      .ok_or(Brc20Error::InsufficientBalance)?;
    if *sender_balance < amount {
      return Err(Brc20Error::InsufficientBalance);
    }
    *sender_balance -= amount;

    let receiver_balance = self.state.balances.entry(to.to_string()).or_insert(0);
    *receiver_balance += amount;

    let proof = ZkProof::generate(from, to, amount, &self.state.prev_state_hash)?;

    self.update_state();

    Ok(proof)
  }

  fn update_state(&mut self) {
    self.state.merkle_root = self.compute_merkle_root();
    self.state.prev_state_hash = self.compute_state_hash();
    self.state.timestamp = Utc::now().timestamp();
  }

  pub fn export_state(&self) -> &TokenState {
    &self.state
  }
}
