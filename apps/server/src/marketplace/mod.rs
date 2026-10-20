pub mod handlers;
pub mod models;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn marketplace_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/marketplace/licenses", get(handlers::list_licenses))
        .route("/api/v1/marketplace/licenses/issue", post(handlers::issue_license))
        .route("/api/v1/marketplace/licenses/verify", post(handlers::verify_license))
}
