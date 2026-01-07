use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRules {
    pub max_per_tx: Option<u64>,
    pub require_identity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingRule {
    pub unlock_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub soulbound: bool,
    pub transfer_rules: TransferRules,
    pub vesting: std::collections::HashMap<String, VestingRule>,
}
