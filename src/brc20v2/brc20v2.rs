use std::collections::BTreeMap;

use bitcoin::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};

use super::{
  errors::Brc20Error,
  identity::IdentityCommitment,
  inscription::Brc20Inscription,
  zk::{ZkProofGenerator, ZkStatement, ZkWitness},
  zk_proof::ZkProof,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenDefinition {
  pub ticker: String,
  pub max_supply: u128,
  pub mint_limit: u128,
  pub decimals: u8,
  pub soulbound: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VestingSchedule {
  pub start_time: u64,
  pub cliff_seconds: u64,
  pub duration_seconds: u64,
  pub total_locked: u128,
}

impl VestingSchedule {
  pub fn validate(&self) -> Result<(), Brc20Error> {
    if self.duration_seconds == 0 {
      return Err(Brc20Error::VestingScheduleInvalid(
        "duration_seconds must be positive".to_string(),
      ));
    }
    if self.cliff_seconds > self.duration_seconds {
      return Err(Brc20Error::VestingScheduleInvalid(
        "cliff_seconds cannot exceed duration_seconds".to_string(),
      ));
    }
    Ok(())
  }

  pub fn unlocked_at(&self, timestamp: u64) -> u128 {
    if timestamp <= self.start_time + self.cliff_seconds {
      return 0;
    }

    let elapsed = timestamp.saturating_sub(self.start_time);
    if elapsed >= self.duration_seconds {
      return self.total_locked;
    }

    let elapsed = elapsed.saturating_sub(self.cliff_seconds);
    let vesting_window = self.duration_seconds.saturating_sub(self.cliff_seconds);
    if vesting_window == 0 {
      return self.total_locked;
    }

    let unlocked = (self.total_locked as u128)
      .saturating_mul(elapsed as u128)
      .saturating_div(vesting_window as u128);
    unlocked.min(self.total_locked)
  }

  pub fn locked_at(&self, timestamp: u64) -> u128 {
    self.total_locked.saturating_sub(self.unlocked_at(timestamp))
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AccountState {
  pub balance: u128,
  pub locked_balance: u128,
  pub vesting: Option<VestingSchedule>,
}

impl AccountState {
  pub fn available_balance(&self, timestamp: u64) -> u128 {
    let locked = match &self.vesting {
      Some(vesting) => vesting.locked_at(timestamp),
      None => self.locked_balance,
    };
    self.balance.saturating_sub(locked)
  }

  fn apply_vesting(&mut self, schedule: VestingSchedule) -> Result<(), Brc20Error> {
    schedule.validate()?;
    if let Some(existing) = &self.vesting {
      if existing.start_time != schedule.start_time
        || existing.cliff_seconds != schedule.cliff_seconds
        || existing.duration_seconds != schedule.duration_seconds
      {
        return Err(Brc20Error::VestingScheduleInvalid(
          "conflicting vesting schedule".to_string(),
        ));
      }
      self.locked_balance = self
        .locked_balance
        .saturating_add(schedule.total_locked);
      let mut merged = existing.clone();
      merged.total_locked = merged.total_locked.saturating_add(schedule.total_locked);
      self.vesting = Some(merged);
    } else {
      self.locked_balance = self.locked_balance.saturating_add(schedule.total_locked);
      self.vesting = Some(schedule);
    }
    Ok(())
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenState {
  pub definition: TokenDefinition,
  pub total_supply: u128,
  pub accounts: BTreeMap<IdentityCommitment, AccountState>,
}

impl TokenState {
  pub fn new(definition: TokenDefinition) -> Self {
    Self {
      definition,
      total_supply: 0,
      accounts: BTreeMap::new(),
    }
  }

  pub fn account_mut(&mut self, identity: &IdentityCommitment) -> &mut AccountState {
    self.accounts.entry(identity.clone()).or_default()
  }

  pub fn merkle_root(&self) -> [u8; 32] {
    let mut leaves = Vec::with_capacity(self.accounts.len());
    for (identity, account) in &self.accounts {
      leaves.push(hash_leaf(identity, account));
    }
    merkle_root(&leaves)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Operation {
  Deploy { definition: TokenDefinition },
  Mint {
    ticker: String,
    to: IdentityCommitment,
    amount: u128,
  },
  MintVested {
    ticker: String,
    to: IdentityCommitment,
    amount: u128,
    vesting: VestingSchedule,
  },
  Transfer {
    ticker: String,
    from: IdentityCommitment,
    to: IdentityCommitment,
    amount: u128,
  },
  SetSoulbound {
    ticker: String,
    soulbound: bool,
  },
}

impl Operation {
  pub fn ticker(&self) -> String {
    match self {
      Self::Deploy { definition } => definition.ticker.clone(),
      Self::Mint { ticker, .. }
      | Self::MintVested { ticker, .. }
      | Self::Transfer { ticker, .. }
      | Self::SetSoulbound { ticker, .. } => ticker.clone(),
    }
  }

  pub fn operation_name(&self) -> &'static str {
    match self {
      Self::Deploy { .. } => "deploy",
      Self::Mint { .. } => "mint",
      Self::MintVested { .. } => "mint_vested",
      Self::Transfer { .. } => "transfer",
      Self::SetSoulbound { .. } => "soulbound",
    }
  }

  pub fn payload_json(&self) -> serde_json::Value {
    match self {
      Self::Deploy { definition } => serde_json::json!({
        "max": definition.max_supply,
        "lim": definition.mint_limit,
        "dec": definition.decimals,
        "soulbound": definition.soulbound,
      }),
      Self::Mint { to, amount, .. } => serde_json::json!({
        "to": to.id,
        "amt": amount,
      }),
      Self::MintVested {
        to,
        amount,
        vesting,
        ..
      } => serde_json::json!({
        "to": to.id,
        "amt": amount,
        "vesting": {
          "start": vesting.start_time,
          "cliff": vesting.cliff_seconds,
          "duration": vesting.duration_seconds,
        },
      }),
      Self::Transfer { from, to, amount, .. } => serde_json::json!({
        "from": from.id,
        "to": to.id,
        "amt": amount,
      }),
      Self::SetSoulbound { soulbound, .. } => serde_json::json!({
        "soulbound": soulbound,
      }),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransitionReceipt {
  pub ticker: String,
  pub merkle_root: [u8; 32],
  pub proof: ZkProof,
  pub inscription: Brc20Inscription,
  pub state_hash: [u8; 32],
}

#[derive(Clone, Debug, Default)]
pub struct Brc20StateMachine {
  pub tokens: BTreeMap<String, TokenState>,
}

impl Brc20StateMachine {
  pub fn apply_operation<G: ZkProofGenerator>(
    &mut self,
    operation: Operation,
    proof_generator: &G,
    timestamp: u64,
  ) -> Result<TransitionReceipt, Brc20Error> {
    let ticker = operation.ticker();
    match &operation {
      Operation::Deploy { definition } => self.deploy(definition.clone())?,
      Operation::Mint { to, amount, .. } => {
        self.mint(&ticker, to, *amount, timestamp)?
      }
      Operation::MintVested {
        to,
        amount,
        vesting,
        ..
      } => self.mint_vested(&ticker, to, *amount, vesting.clone(), timestamp)?,
      Operation::Transfer {
        from, to, amount, ..
      } => self.transfer(&ticker, from, to, *amount, timestamp)?,
      Operation::SetSoulbound { soulbound, .. } => {
        self.set_soulbound(&ticker, *soulbound)?
      }
    };

    let token_state = self
      .tokens
      .get(&ticker)
      .ok_or_else(|| Brc20Error::TokenNotFound(ticker.clone()))?;
    let merkle_root = token_state.merkle_root();
    let (statement, witness) = self.zk_inputs(&operation, token_state, timestamp)?;
    let proof = proof_generator.generate(&statement, &witness);
    if !proof_generator.verify(&statement, &proof) {
      return Err(Brc20Error::ProofVerificationFailed(
        "proof generator failed to verify".to_string(),
      ));
    }

    let inscription = Brc20Inscription::from_operation(&operation, merkle_root);
    let state_hash = hash_state(token_state, merkle_root);

    Ok(TransitionReceipt {
      ticker,
      merkle_root,
      proof,
      inscription,
      state_hash,
    })
  }

  fn deploy(&mut self, definition: TokenDefinition) -> Result<(), Brc20Error> {
    if self.tokens.contains_key(&definition.ticker) {
      return Err(Brc20Error::TokenAlreadyExists(definition.ticker));
    }
    self
      .tokens
      .insert(definition.ticker.clone(), TokenState::new(definition));
    Ok(())
  }

  fn mint(
    &mut self,
    ticker: &str,
    to: &IdentityCommitment,
    amount: u128,
    timestamp: u64,
  ) -> Result<(), Brc20Error> {
    let token = self
      .tokens
      .get_mut(ticker)
      .ok_or_else(|| Brc20Error::TokenNotFound(ticker.to_string()))?;

    if amount > token.definition.mint_limit {
      return Err(Brc20Error::MintLimitExceeded {
        limit: token.definition.mint_limit,
        requested: amount,
      });
    }

    let attempted_total = token.total_supply.saturating_add(amount);
    if attempted_total > token.definition.max_supply {
      return Err(Brc20Error::MaxSupplyExceeded {
        max_supply: token.definition.max_supply,
        attempted_total,
      });
    }

    let account = token.account_mut(to);
    account.balance = account.balance.saturating_add(amount);
    account.locked_balance = account
      .vesting
      .as_ref()
      .map(|vesting| vesting.locked_at(timestamp))
      .unwrap_or(account.locked_balance);

    token.total_supply = attempted_total;
    Ok(())
  }

  fn mint_vested(
    &mut self,
    ticker: &str,
    to: &IdentityCommitment,
    amount: u128,
    vesting: VestingSchedule,
    timestamp: u64,
  ) -> Result<(), Brc20Error> {
    self.mint(ticker, to, amount, timestamp)?;
    let token = self
      .tokens
      .get_mut(ticker)
      .ok_or_else(|| Brc20Error::TokenNotFound(ticker.to_string()))?;
    let account = token.account_mut(to);
    account.apply_vesting(VestingSchedule {
      total_locked: amount,
      ..vesting
    })?;
    Ok(())
  }

  fn transfer(
    &mut self,
    ticker: &str,
    from: &IdentityCommitment,
    to: &IdentityCommitment,
    amount: u128,
    timestamp: u64,
  ) -> Result<(), Brc20Error> {
    let token = self
      .tokens
      .get_mut(ticker)
      .ok_or_else(|| Brc20Error::TokenNotFound(ticker.to_string()))?;

    if token.definition.soulbound {
      return Err(Brc20Error::SoulboundTransferDenied(
        ticker.to_string(),
      ));
    }

    let sender = token.account_mut(from);
    let available = sender.available_balance(timestamp);
    if available < amount {
      return Err(Brc20Error::InsufficientBalance {
        available,
        required: amount,
      });
    }

    sender.balance = sender.balance.saturating_sub(amount);
    let receiver = token.account_mut(to);
    receiver.balance = receiver.balance.saturating_add(amount);
    Ok(())
  }

  fn set_soulbound(&mut self, ticker: &str, soulbound: bool) -> Result<(), Brc20Error> {
    let token = self
      .tokens
      .get_mut(ticker)
      .ok_or_else(|| Brc20Error::TokenNotFound(ticker.to_string()))?;
    token.definition.soulbound = soulbound;
    Ok(())
  }

  fn zk_inputs(
    &self,
    operation: &Operation,
    token: &TokenState,
    timestamp: u64,
  ) -> Result<(ZkStatement, ZkWitness), Brc20Error> {
    let merkle_root = token.merkle_root();
    match operation {
      Operation::Deploy { .. } | Operation::SetSoulbound { .. } => Ok((
        ZkStatement {
          operation: operation.operation_name().to_string(),
          token: token.definition.ticker.clone(),
          from: None,
          to: None,
          amount: 0,
          merkle_root,
        },
        ZkWitness {
          from_balance: 0,
          to_balance: 0,
        },
      )),
      Operation::Mint { to, amount, .. } | Operation::MintVested { to, amount, .. } => {
        let account = token
          .accounts
          .get(to)
          .cloned()
          .unwrap_or_default();
        Ok((
          ZkStatement {
            operation: operation.operation_name().to_string(),
            token: token.definition.ticker.clone(),
            from: None,
            to: Some(to.commitment),
            amount: *amount,
            merkle_root,
          },
          ZkWitness {
            from_balance: 0,
            to_balance: account.available_balance(timestamp),
          },
        ))
      }
      Operation::Transfer {
        from, to, amount, ..
      } => {
        let from_balance = token
          .accounts
          .get(from)
          .map(|account| account.available_balance(timestamp))
          .unwrap_or_default();
        let to_balance = token
          .accounts
          .get(to)
          .map(|account| account.available_balance(timestamp))
          .unwrap_or_default();
        Ok((
          ZkStatement {
            operation: operation.operation_name().to_string(),
            token: token.definition.ticker.clone(),
            from: Some(from.commitment),
            to: Some(to.commitment),
            amount: *amount,
            merkle_root,
          },
          ZkWitness {
            from_balance,
            to_balance,
          },
        ))
      }
    }
  }
}

fn hash_leaf(identity: &IdentityCommitment, account: &AccountState) -> [u8; 32] {
  let mut data = Vec::with_capacity(32 + 16 + 16 + 1);
  data.extend_from_slice(&identity.commitment);
  data.extend_from_slice(&account.balance.to_be_bytes());
  data.extend_from_slice(&account.locked_balance.to_be_bytes());
  data.push(account.vesting.is_some() as u8);
  sha256::Hash::hash(&data).to_byte_array()
}

fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
  if leaves.is_empty() {
    return sha256::Hash::hash(&[]).to_byte_array();
  }

  let mut level: Vec<[u8; 32]> = leaves.to_vec();
  while level.len() > 1 {
    let mut next = Vec::with_capacity((level.len() + 1) / 2);
    let mut iter = level.chunks(2);
    while let Some(pair) = iter.next() {
      let right = if pair.len() == 2 { pair[1] } else { pair[0] };
      next.push(hash_pair(pair[0], right));
    }
    level = next;
  }
  level[0]
}

fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
  let mut data = Vec::with_capacity(64);
  data.extend_from_slice(&left);
  data.extend_from_slice(&right);
  sha256::Hash::hash(&data).to_byte_array()
}

fn hash_state(token: &TokenState, merkle_root: [u8; 32]) -> [u8; 32] {
  let mut data = Vec::new();
  data.extend_from_slice(token.definition.ticker.as_bytes());
  data.extend_from_slice(&token.total_supply.to_be_bytes());
  data.extend_from_slice(&token.definition.max_supply.to_be_bytes());
  data.extend_from_slice(&merkle_root);
  sha256::Hash::hash(&data).to_byte_array()
}
