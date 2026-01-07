use serde::Serialize;

use crate::zk::ZkProof;

#[derive(Serialize)]
pub struct Inscription<'a> {
    pub protocol: &'static str,
    pub token: &'a str,
    pub action: &'a str,
    pub state_hash: &'a str,
    pub merkle_root: &'a str,
    pub proof: Option<&'a ZkProof>,
}

impl<'a> Inscription<'a> {
    pub fn new(
        token: &'a str,
        action: &'a str,
        state_hash: &'a str,
        merkle_root: &'a str,
        proof: Option<&'a ZkProof>,
    ) -> Self {
        Self {
            protocol: "brc20v2",
            token,
            action,
            state_hash,
            merkle_root,
            proof,
        }
    }
}
