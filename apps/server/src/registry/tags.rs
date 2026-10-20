use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct TagsListResponse {
    pub name: String,
    pub tags: Vec<String>,
}

pub async fn list_tags(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT t.name
        FROM tags t
        JOIN repositories r ON r.id = t.repository_id
        WHERE r.namespace = $1 AND r.name = $2
        ORDER BY t.updated_at DESC
        "#,
        namespace,
        name
    )
    .fetch_all(&state.db)
    .await?;

    let tags = rows.into_iter().map(|r| r.name).collect();
    let body = TagsListResponse {
        name: format!("{}/{}", namespace, name),
        tags,
    };

    let mut res = Json(body).into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    Ok(res)
}
