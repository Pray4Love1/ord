use thiserror::Error;

#[derive(Error, Debug)]
pub enum Brc20Error {
    #[error("identity verification failed")]
    IdentityFailed,

    #[error("insufficient balance")]
    InsufficientBalance,

    #[error("token is soulbound")]
    Soulbound,

    #[error("vesting locked until block {0}")]
    VestingLocked(u64),

    #[error("amount exceeds max per tx")]
    MaxTransferExceeded,

    #[error("invalid zk proof")]
    InvalidProof,

    #[error("relay error: {0}")]
    Relay(String),
}
