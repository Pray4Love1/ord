#[allow(dead_code)]
pub fn prepare_inscription_payload(payload: &str) -> Vec<u8> {
    payload.as_bytes().to_vec()
}
