use rand::RngCore;
use sha2::{Digest, Sha256};

pub struct GeneratedToken {
    pub raw_token: String,
    pub prefix: String,
    pub token_hash: String,
}

pub fn generate_personal_access_token() -> GeneratedToken {
    let mut bytes = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut bytes);
    let hex_suffix = hex::encode(bytes);
    let raw_token = format!("vyh_{}", hex_suffix);
    let prefix = raw_token[..8].to_string(); // e.g. "vyh_9a8b"
    let token_hash = hash_token(&raw_token);

    GeneratedToken {
        raw_token,
        prefix,
        token_hash,
    }
}

pub fn hash_token(raw_token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_token.as_bytes());
    hex::encode(hasher.finalize())
}
