use sha2::{Digest, Sha256};

pub fn merkle_root(leaves: &[String]) -> String {
    if leaves.is_empty() {
        return String::new();
    }

    let mut nodes: Vec<String> = leaves
        .iter()
        .map(|l| {
            let mut h = Sha256::new();
            h.update(l.as_bytes());
            hex::encode(h.finalize())
        })
        .collect();

    while nodes.len() > 1 {
        let mut next = vec![];
        for i in (0..nodes.len()).step_by(2) {
            let left = &nodes[i];
            let right = nodes.get(i + 1).unwrap_or(left);
            let mut h = Sha256::new();
            h.update(format!("{}{}", left, right));
            next.push(hex::encode(h.finalize()));
        }
        nodes = next;
    }

    nodes[0].clone()
}
