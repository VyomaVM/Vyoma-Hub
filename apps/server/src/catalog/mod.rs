pub mod handlers;
pub mod models;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn catalog_routes() -> Router<AppState> {
    Router::new()
        // Catalog Explorer
        .route("/api/v1/catalog", get(handlers::list_catalog))
        .route("/api/v1/repos/:namespace/:name", get(handlers::get_repo))
        // Certified Kernel Catalog
        .route("/api/v1/kernels", get(handlers::list_kernels))
        .route("/api/v1/kernels/:name/:version", get(handlers::get_kernel))
        // Native VMIF Appliance Engine (Section 3.1)
        .route("/v2/vyoma/appliances/:name/manifest", get(handlers::get_vmif_manifest))
        .route("/v2/vyoma/kernels/:profile/:version", get(handlers::get_vmif_kernel))
        .route("/api/v1/appliances/upload", post(handlers::upload_appliance))
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Signer, SigningKey, Verifier};
    use rand::rngs::OsRng;

    #[test]
    fn test_ed25519_kernel_signature_verification() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let name = "database";
        let version = "6.6.30-vyoma";
        let vmlinuz_digest = "sha256:7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b";
        let config_digest = "sha256:11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff";

        let payload = format!("{}:{}:{}:{}", name, version, vmlinuz_digest, config_digest);
        let signature = signing_key.sign(payload.as_bytes());

        // Verification
        assert!(verifying_key.verify(payload.as_bytes(), &signature).is_ok());

        // Tamper detection test
        let tampered_payload = format!("{}:{}:{}:{}", name, "6.6.31-tampered", vmlinuz_digest, config_digest);
        assert!(verifying_key.verify(tampered_payload.as_bytes(), &signature).is_err());
    }
}
