mod config;
mod errors;
mod identity;
mod inscription;
mod merkle;
mod state;
mod zk;

pub mod relay;

use config::TokenConfig;
use errors::Brc20Error;
use state::TokenState;
use zk::{generate_proof, ZkProof};

pub struct Brc20v2 {
    pub token: String,
    pub config: TokenConfig,
    pub state: TokenState,
}

impl Brc20v2 {
    pub fn new(token: &str, config: TokenConfig) -> Self {
        Self {
            token: token.to_string(),
            config,
            state: TokenState::new(),
        }
    }

    pub fn mint(&mut self, to: &str, amount: u64) {
        *self.state.balances.entry(to.into()).or_insert(0) += amount;
        self.state.update();
    }

    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: u64,
        current_block: u64,
        identity_verified: bool,
    ) -> Result<ZkProof, Brc20Error> {
        if self.config.soulbound {
            return Err(Brc20Error::Soulbound);
        }

        if let Some(v) = self.config.vesting.get(from) {
            if current_block < v.unlock_block {
                return Err(Brc20Error::VestingLocked(v.unlock_block));
            }
        }

        if let Some(max) = self.config.transfer_rules.max_per_tx {
            if amount > max {
                return Err(Brc20Error::MaxTransferExceeded);
            }
        }

        let balance = self
            .state
            .balances
            .get_mut(from)
            .ok_or(Brc20Error::InsufficientBalance)?;

        if *balance < amount {
            return Err(Brc20Error::InsufficientBalance);
        }

        *balance -= amount;
        *self.state.balances.entry(to.into()).or_insert(0) += amount;

        let proof = generate_proof(
            from,
            to,
            amount,
            &self.state.prev_state_hash,
            identity_verified,
        )?;

        self.state.update();
        Ok(proof)
    }
}
