pub trait IdentityVerifier {
    fn verify(&self, subject: &str) -> bool;
}

pub struct SoulSyncVerifier;

impl IdentityVerifier for SoulSyncVerifier {
    fn verify(&self, _subject: &str) -> bool {
        true
    }
}
