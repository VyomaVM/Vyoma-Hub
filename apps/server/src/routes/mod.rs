pub mod auth;
pub mod health;
pub mod oci_auth;

use crate::state::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers([
            axum::http::HeaderName::from_static("docker-distribution-api-version"),
            axum::http::HeaderName::from_static("docker-content-digest"),
            axum::http::HeaderName::from_static("location"),
        ]);

    Router::new()
        // Health check
        .route("/health", get(health::health_check))
        // OCI Registry v2 Handshake & Token Endpoints
        .route("/v2/", get(oci_auth::oci_ping))
        .route("/v2/token", get(oci_auth::oci_token))
        // Hub REST API v1: Auth & User Tokens
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/me", get(auth::me))
        .route("/api/v1/auth/tokens", post(auth::create_token).get(auth::list_tokens))
        .route("/api/v1/auth/tokens/:id", delete(auth::delete_token))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
