use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde_json::Value;
use sha2::{Digest, Sha256};

// HEAD /v2/:namespace/:name/manifests/:reference
pub async fn head_manifest(
    State(state): State<AppState>,
    Path((namespace, name, reference)): Path<(String, String, String)>,
) -> Result<Response, AppError> {
    let manifest_digest = resolve_reference(&state, &namespace, &name, &reference).await?;

    let row = sqlx::query!(
        "SELECT size_bytes, media_type FROM manifests WHERE digest = $1",
        manifest_digest
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Manifest not found".to_string()))?;

    let mut res = StatusCode::OK.into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    res.headers_mut().insert(
        "Docker-Content-Digest",
        HeaderValue::from_str(&manifest_digest).map_err(|_| AppError::Internal(anyhow::anyhow!("Digest error")))?,
    );
    res.headers_mut().insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&row.size_bytes.to_string()).unwrap(),
    );
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&row.media_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/vnd.oci.image.manifest.v1+json")),
    );

    Ok(res)
}

// GET /v2/:namespace/:name/manifests/:reference
pub async fn get_manifest(
    State(state): State<AppState>,
    Path((namespace, name, reference)): Path<(String, String, String)>,
) -> Result<Response, AppError> {
    let manifest_digest = resolve_reference(&state, &namespace, &name, &reference).await?;

    let row = sqlx::query!(
        "SELECT size_bytes, media_type, payload FROM manifests WHERE digest = $1",
        manifest_digest
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Manifest not found".to_string()))?;

    // Increment pull counter on repository
    let ns_clone = namespace.clone();
    let name_clone = name.clone();
    let pool = state.db.clone();
    tokio::spawn(async move {
        sqlx::query!(
            "UPDATE repositories SET pull_count = pull_count + 1 WHERE namespace = $1 AND name = $2",
            ns_clone,
            name_clone
        )
        .execute(&pool)
        .await
        .ok();
    });

    let payload_bytes = serde_json::to_vec(&row.payload)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Serialization error: {}", e)))?;

    let mut res = (StatusCode::OK, payload_bytes).into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    res.headers_mut().insert(
        "Docker-Content-Digest",
        HeaderValue::from_str(&manifest_digest).map_err(|_| AppError::Internal(anyhow::anyhow!("Digest error")))?,
    );
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&row.media_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/vnd.oci.image.manifest.v1+json")),
    );

    Ok(res)
}

// PUT /v2/:namespace/:name/manifests/:reference
pub async fn put_manifest(
    State(state): State<AppState>,
    Path((namespace, name, reference)): Path<(String, String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    let media_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/vnd.oci.image.manifest.v1+json")
        .to_string();

    let mut hasher = Sha256::new();
    hasher.update(&body);
    let calculated_digest = format!("sha256:{}", hex::encode(hasher.finalize()));

    let payload: Value = serde_json::from_slice(&body).map_err(|e| {
        AppError::BadRequest(format!("Invalid manifest JSON payload: {}", e))
    })?;

    // Extract config digest if present
    let config_digest = payload
        .get("config")
        .and_then(|c| c.get("digest"))
        .and_then(|d| d.as_str())
        .map(|s| s.to_string());

    // 1. Ensure Repository exists (auto-create if new)
    let repo_id = match sqlx::query!(
        "SELECT id FROM repositories WHERE namespace = $1 AND name = $2",
        namespace,
        name
    )
    .fetch_optional(&state.db)
    .await?
    {
        Some(r) => r.id,
        None => {
            sqlx::query!(
                r#"
                INSERT INTO repositories (namespace, name, description, is_public)
                VALUES ($1, $2, $3, TRUE)
                RETURNING id
                "#,
                namespace,
                name,
                format!("Appliance repository for {}/{}", namespace, name)
            )
            .fetch_one(&state.db)
            .await?
            .id
        }
    };

    let size_bytes = body.len() as i64;

    // 2. Upsert Manifest
    sqlx::query!(
        r#"
        INSERT INTO manifests (digest, repository_id, media_type, size_bytes, config_digest, payload)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (digest) DO UPDATE SET
            media_type = EXCLUDED.media_type,
            size_bytes = EXCLUDED.size_bytes,
            config_digest = EXCLUDED.config_digest,
            payload = EXCLUDED.payload
        "#,
        calculated_digest,
        repo_id,
        media_type,
        size_bytes,
        config_digest,
        payload
    )
    .execute(&state.db)
    .await?;

    // 3. Associate Layers if present in JSON
    if let Some(layers) = payload.get("layers").and_then(|l| l.as_array()) {
        for (i, layer) in layers.iter().enumerate() {
            if let Some(layer_digest) = layer.get("digest").and_then(|d| d.as_str()) {
                sqlx::query!(
                    r#"
                    INSERT INTO manifest_blobs (manifest_digest, blob_digest, ordinal)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (manifest_digest, blob_digest) DO NOTHING
                    "#,
                    calculated_digest,
                    layer_digest,
                    i as i32
                )
                .execute(&state.db)
                .await
                .ok();
            }
        }
    }

    // 4. Update Tag if reference is a tag name (not a digest)
    if !reference.starts_with("sha256:") {
        sqlx::query!(
            r#"
            INSERT INTO tags (repository_id, name, manifest_digest)
            VALUES ($1, $2, $3)
            ON CONFLICT (repository_id, name) DO UPDATE SET
                manifest_digest = EXCLUDED.manifest_digest,
                updated_at = NOW()
            "#,
            repo_id,
            reference,
            calculated_digest
        )
        .execute(&state.db)
        .await?;
    }

    let location = format!(
        "/v2/{}/{}/manifests/{}",
        namespace, name, calculated_digest
    );
    let mut res = StatusCode::CREATED.into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    res.headers_mut().insert(
        header::LOCATION,
        HeaderValue::from_str(&location).unwrap(),
    );
    res.headers_mut().insert(
        "Docker-Content-Digest",
        HeaderValue::from_str(&calculated_digest).unwrap(),
    );

    Ok(res)
}

async fn resolve_reference(
    state: &AppState,
    namespace: &str,
    name: &str,
    reference: &str,
) -> Result<String, AppError> {
    if reference.starts_with("sha256:") {
        return Ok(reference.to_string());
    }

    // Lookup tag
    let tag_row = sqlx::query!(
        r#"
        SELECT t.manifest_digest
        FROM tags t
        JOIN repositories r ON r.id = t.repository_id
        WHERE r.namespace = $1 AND r.name = $2 AND t.name = $3
        "#,
        namespace,
        name,
        reference
    )
    .fetch_optional(&state.db)
    .await?;

    if let Some(tag) = tag_row {
        Ok(tag.manifest_digest)
    } else {
        Err(AppError::NotFound(format!(
            "Tag '{}' not found in repository '{}/{}'",
            reference, namespace, name
        )))
    }
}
