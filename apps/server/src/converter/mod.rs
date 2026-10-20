pub mod handlers;
pub mod models;
pub mod worker;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn converter_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/converter/jobs", post(handlers::submit_conversion_job).get(handlers::list_conversion_jobs))
        .route("/api/v1/converter/jobs/:id", get(handlers::get_conversion_job))
}
