use sha3::{Digest, Sha3_256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hash([u8; 32]);

impl Hash {
  pub fn as_bytes(&self) -> &[u8; 32] {
    &self.0
  }

  pub fn to_hex(&self) -> String {
    hex::encode(self.0)
  }
}

pub struct Hasher {
  inner: Sha3_256,
}

impl Hasher {
  pub fn new() -> Self {
    Self {
      inner: Sha3_256::new(),
    }
  }

  pub fn update(&mut self, data: &[u8]) {
    self.inner.update(data);
  }

  pub fn finalize(self) -> Hash {
    let bytes = self.inner.finalize();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    Hash(array)
  }
}

pub fn hash(data: &[u8]) -> Hash {
  let mut hasher = Hasher::new();
  hasher.update(data);
  hasher.finalize()
}
