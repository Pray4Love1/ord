use serde::{Deserialize, Serialize};

use super::brc20v2::Operation;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Brc20Inscription {
  pub content_type: String,
  pub body: Vec<u8>,
}

impl Brc20Inscription {
  pub fn from_operation(operation: &Operation, merkle_root: [u8; 32]) -> Self {
    let payload = serde_json::json!({
      "p": "brc-20-v2",
      "op": operation.operation_name(),
      "tick": operation.ticker(),
      "body": operation.payload_json(),
      "root": hex::encode(merkle_root),
    });

    Self {
      content_type: "application/json".to_string(),
      body: serde_json::to_vec(&payload).unwrap_or_default(),
    }
  }
}
