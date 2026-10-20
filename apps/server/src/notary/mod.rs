pub mod handlers;
pub mod merkle;
pub mod models;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn notary_routes() -> Router<AppState> {
    Router::new()
        // Transparency Log & Notary Attestation
        .route("/api/v1/notary/log", get(handlers::get_transparency_log))
        .route("/api/v1/notary/verify", post(handlers::verify_attestation))
        // Vulnerability Scan Reports
        .route("/api/v1/security/scans/:namespace/:name", get(handlers::get_security_scan))
        .route("/api/v1/security/scan", post(handlers::trigger_scan))
}

#[cfg(test)]
mod tests {
    use super::merkle::*;

    #[test]
    fn test_transparency_merkle_tree() {
        let leaf1 = calculate_leaf_hash(1, "vyoma/postgres", "sha256:abcd", "sig:1234");
        let leaf2 = calculate_leaf_hash(2, "vyoma/nginx", "sha256:ef01", "sig:5678");

        let root = calculate_merkle_root(&[leaf1, leaf2]);
        assert!(root.starts_with("sha256:"));
    }
}
