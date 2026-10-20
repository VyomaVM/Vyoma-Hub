pub mod blobs;
pub mod manifests;
pub mod tags;

use crate::state::AppState;
use axum::{
    routing::{get, head, patch, post},
    Router,
};

pub fn registry_routes(_state: AppState) -> Router<AppState> {
    Router::new()
        // Blobs
        .route("/v2/:namespace/:name/blobs/:digest", head(blobs::head_blob).get(blobs::get_blob))
        .route("/v2/:namespace/:name/blobs/uploads/", post(blobs::start_blob_upload))
        .route(
            "/v2/:namespace/:name/blobs/uploads/:upload_id",
            patch(blobs::patch_blob_upload).put(blobs::finish_blob_upload),
        )
        // Manifests
        .route(
            "/v2/:namespace/:name/manifests/:reference",
            head(manifests::head_manifest)
                .get(manifests::get_manifest)
                .put(manifests::put_manifest),
        )
        // Tags
        .route("/v2/:namespace/:name/tags/list", get(tags::list_tags))
}
