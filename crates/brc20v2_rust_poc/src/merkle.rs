use sha2::{Digest, Sha256};

pub fn merkle_root(leaves: &[String]) -> String {
    if leaves.is_empty() {
        return String::new();
    }

    let mut nodes: Vec<String> = leaves
        .iter()
        .map(|value| hex::encode(Sha256::digest(value.as_bytes())))
        .collect();

    while nodes.len() > 1 {
        let mut next = Vec::new();
        for pair in nodes.chunks(2) {
            let combined = if pair.len() == 2 {
                format!("{}{}", pair[0], pair[1])
            } else {
                format!("{}{}", pair[0], pair[0])
            };
            next.push(hex::encode(Sha256::digest(combined.as_bytes())));
        }
        nodes = next;
    }

    nodes[0].clone()
}
