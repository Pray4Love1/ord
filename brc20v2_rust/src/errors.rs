use std::fmt;

#[derive(Debug)]
pub enum Brc20Error {
    Relay(String),
}

impl fmt::Display for Brc20Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relay(message) => write!(f, "Relay error: {message}"),
        }
    }
}

impl std::error::Error for Brc20Error {}
