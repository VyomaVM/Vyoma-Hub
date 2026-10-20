use sha2::{Digest, Sha256};

pub fn calculate_leaf_hash(sequence_number: i64, target_name: &str, digest: &str, signature: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}:{}:{}:{}", sequence_number, target_name, digest, signature).as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

pub fn calculate_merkle_root(leaves: &[String]) -> String {
    if leaves.is_empty() {
        return "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string();
    }

    let mut current_layer: Vec<String> = leaves.to_vec();

    while current_layer.len() > 1 {
        let mut next_layer = Vec::new();

        for chunk in current_layer.chunks(2) {
            let left = &chunk[0];
            let right = if chunk.len() > 1 { &chunk[1] } else { left };

            let mut hasher = Sha256::new();
            hasher.update(left.as_bytes());
            hasher.update(right.as_bytes());
            next_layer.push(format!("sha256:{:x}", hasher.finalize()));
        }

        current_layer = next_layer;
    }

    current_layer[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_leaf_deterministic() {
        let leaf1 = calculate_leaf_hash(1, "kernel:database", "sha256:1111", "sig:aaaa");
        let leaf2 = calculate_leaf_hash(1, "kernel:database", "sha256:1111", "sig:aaaa");
        assert_eq!(leaf1, leaf2);
        assert!(leaf1.starts_with("sha256:"));
    }

    #[test]
    fn test_merkle_root_pairwise() {
        let leaves = vec![
            calculate_leaf_hash(1, "kernel:database", "sha256:1111", "sig:aaaa"),
            calculate_leaf_hash(2, "kernel:edge", "sha256:2222", "sig:bbbb"),
            calculate_leaf_hash(3, "kernel:ai", "sha256:3333", "sig:cccc"),
        ];

        let root = calculate_merkle_root(&leaves);
        assert!(root.starts_with("sha256:"));

        // Modifying any leaf changes root
        let mut tampered = leaves.clone();
        tampered[1] = calculate_leaf_hash(2, "kernel:edge-tampered", "sha256:2222", "sig:bbbb");
        let tampered_root = calculate_merkle_root(&tampered);
        assert_ne!(root, tampered_root);
    }
}
