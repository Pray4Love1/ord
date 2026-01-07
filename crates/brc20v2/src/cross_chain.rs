use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainRelay {
    pub source_chain: String,
    pub destination_chain: String,
    pub source_tx: String,
    pub destination_tx: Option<String>,
    pub block_height: u64,
    pub relayer: String,
}

impl CrossChainRelay {
    pub fn new(
        source_chain: &str,
        destination_chain: &str,
        source_tx: &str,
        block_height: u64,
        relayer: &str,
    ) -> Self {
        Self {
            source_chain: source_chain.to_string(),
            destination_chain: destination_chain.to_string(),
            source_tx: source_tx.to_string(),
            destination_tx: None,
            block_height,
            relayer: relayer.to_string(),
        }
    }

    pub fn mark_confirmed(&mut self, destination_tx: &str) {
        self.destination_tx = Some(destination_tx.to_string());
    }
}
