#[derive(Debug, Clone)]
pub struct RelayPayload {
    pub destination: String,
    pub data: Vec<u8>,
}
