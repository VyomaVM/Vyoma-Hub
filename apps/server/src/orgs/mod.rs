pub mod handlers;
pub mod models;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn org_routes() -> Router<AppState> {
    Router::new()
        // Organizations
        .route("/api/v1/orgs", post(handlers::create_organization).get(handlers::list_organizations))
        .route("/api/v1/orgs/:name", get(handlers::get_organization))
        // Organization Members & RBAC
        .route("/api/v1/orgs/:name/members", get(handlers::list_members).post(handlers::add_member))
        // Webhooks
        .route("/api/v1/orgs/:name/webhooks", get(handlers::list_webhooks).post(handlers::create_webhook))
        .route("/api/v1/webhooks/:id/test", post(handlers::test_webhook))
}

#[cfg(test)]
mod tests {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    #[test]
    fn test_webhook_hmac_signing() {
        let secret = "whsec_test_secret_123456";
        let payload = r#"{"event":"push","repo":"vyoma/postgres"}"#;

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        assert_eq!(signature.len(), 64);
    }
}
