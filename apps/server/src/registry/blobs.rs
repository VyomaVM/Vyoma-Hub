use crate::{error::AppError, state::AppState};
use aws_sdk_s3::primitives::ByteStream;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UploadQuery {
    pub digest: Option<String>,
}

fn uploads_dir() -> PathBuf {
    PathBuf::from("/tmp/vyoma-uploads")
}

// HEAD /v2/:namespace/:name/blobs/:digest
pub async fn head_blob(
    State(state): State<AppState>,
    Path((_namespace, _name, digest)): Path<(String, String, String)>,
) -> Result<Response, AppError> {
    let row = sqlx::query!(
        "SELECT size_bytes, media_type FROM blobs WHERE digest = $1",
        digest
    )
    .fetch_optional(&state.db)
    .await?;

    if let Some(blob) = row {
        let mut res = StatusCode::OK.into_response();
        res.headers_mut().insert(
            "Docker-Distribution-API-Version",
            HeaderValue::from_static("registry/2.0"),
        );
        res.headers_mut().insert(
            "Docker-Content-Digest",
            HeaderValue::from_str(&digest).map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid digest header")))?,
        );
        res.headers_mut().insert(
            header::CONTENT_LENGTH,
            HeaderValue::from_str(&blob.size_bytes.to_string()).unwrap(),
        );
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&blob.media_type)
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
        );
        Ok(res)
    } else {
        Err(AppError::NotFound("Blob not found".to_string()))
    }
}

// GET /v2/:namespace/:name/blobs/:digest
pub async fn get_blob(
    State(state): State<AppState>,
    Path((_namespace, _name, digest)): Path<(String, String, String)>,
) -> Result<Response, AppError> {
    let row = sqlx::query!(
        "SELECT size_bytes, media_type, storage_path FROM blobs WHERE digest = $1",
        digest
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Blob not found".to_string()))?;

    let s3_resp = state
        .s3
        .get_object()
        .bucket(&state.config.s3_bucket)
        .key(&row.storage_path)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch blob from S3: {:?}", e);
            AppError::NotFound("Blob object missing in storage".to_string())
        })?;

    let bytes = s3_resp
        .body
        .collect()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("S3 streaming error: {:?}", e)))?
        .into_bytes();

    let body = Body::from(bytes);

    let mut res = (StatusCode::OK, body).into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    res.headers_mut().insert(
        "Docker-Content-Digest",
        HeaderValue::from_str(&digest).map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid digest header")))?,
    );
    res.headers_mut().insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&row.size_bytes.to_string()).unwrap(),
    );
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&row.media_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );

    Ok(res)
}

// POST /v2/:namespace/:name/blobs/uploads/
pub async fn start_blob_upload(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
    Query(query): Query<UploadQuery>,
    body: Bytes,
) -> Result<Response, AppError> {
    let upload_id = Uuid::new_v4().to_string();
    let dir = uploads_dir();
    fs::create_dir_all(&dir).await.ok();
    let upload_path = dir.join(&upload_id);

    // If monolithic upload with digest provided
    if let Some(expected_digest) = query.digest {
        let mut hasher = Sha256::new();
        hasher.update(&body);
        let calculated_digest = format!("sha256:{}", hex::encode(hasher.finalize()));

        if calculated_digest != expected_digest {
            return Err(AppError::BadRequest(format!(
                "Digest mismatch: expected {}, got {}",
                expected_digest, calculated_digest
            )));
        }

        let size_bytes = body.len() as i64;
        let storage_path = format!("blobs/{}", calculated_digest);

        // Upload to S3
        state
            .s3
            .put_object()
            .bucket(&state.config.s3_bucket)
            .key(&storage_path)
            .body(ByteStream::from(body))
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("S3 upload error: {:?}", e)))?;

        // Record blob in DB
        sqlx::query!(
            r#"
            INSERT INTO blobs (digest, size_bytes, media_type, storage_path)
            VALUES ($1, $2, 'application/octet-stream', $3)
            ON CONFLICT (digest) DO NOTHING
            "#,
            calculated_digest,
            size_bytes,
            storage_path
        )
        .execute(&state.db)
        .await?;

        let location = format!("/v2/{}/{}/blobs/{}", namespace, name, calculated_digest);
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
        return Ok(res);
    }

    // Otherwise initiate chunked session
    fs::write(&upload_path, &body).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to initialize upload buffer: {}", e))
    })?;

    let location = format!(
        "/v2/{}/{}/blobs/uploads/{}",
        namespace, name, upload_id
    );
    let mut res = StatusCode::ACCEPTED.into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    res.headers_mut().insert(
        header::LOCATION,
        HeaderValue::from_str(&location).unwrap(),
    );
    res.headers_mut().insert(
        "Range",
        HeaderValue::from_str(&format!("0-{}", body.len())).unwrap(),
    );
    res.headers_mut().insert(
        "Docker-Upload-UUID",
        HeaderValue::from_str(&upload_id).unwrap(),
    );

    Ok(res)
}

// PATCH /v2/:namespace/:name/blobs/uploads/:upload_id
pub async fn patch_blob_upload(
    Path((namespace, name, upload_id)): Path<(String, String, String)>,
    body: Bytes,
) -> Result<Response, AppError> {
    let upload_path = uploads_dir().join(&upload_id);
    if !upload_path.exists() {
        return Err(AppError::NotFound("Upload session not found".to_string()));
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&upload_path)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to open upload buffer: {}", e)))?;

    file.write_all(&body).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to append upload chunk: {}", e))
    })?;
    file.flush().await.ok();

    let metadata = fs::metadata(&upload_path).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to read buffer metadata: {}", e))
    })?;

    let location = format!(
        "/v2/{}/{}/blobs/uploads/{}",
        namespace, name, upload_id
    );
    let mut res = StatusCode::ACCEPTED.into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    res.headers_mut().insert(
        header::LOCATION,
        HeaderValue::from_str(&location).unwrap(),
    );
    res.headers_mut().insert(
        "Range",
        HeaderValue::from_str(&format!("0-{}", metadata.len())).unwrap(),
    );
    res.headers_mut().insert(
        "Docker-Upload-UUID",
        HeaderValue::from_str(&upload_id).unwrap(),
    );

    Ok(res)
}

// PUT /v2/:namespace/:name/blobs/uploads/:upload_id?digest=<digest>
pub async fn finish_blob_upload(
    State(state): State<AppState>,
    Path((namespace, name, upload_id)): Path<(String, String, String)>,
    Query(query): Query<UploadQuery>,
    body: Bytes,
) -> Result<Response, AppError> {
    let expected_digest = query.digest.ok_or_else(|| {
        AppError::BadRequest("Missing required 'digest' query parameter".to_string())
    })?;

    let upload_path = uploads_dir().join(&upload_id);
    if !upload_path.exists() && body.is_empty() {
        return Err(AppError::NotFound("Upload session not found".to_string()));
    }

    if !body.is_empty() {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&upload_path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Upload file error: {}", e)))?;
        file.write_all(&body).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to write chunk: {}", e))
        })?;
        file.flush().await.ok();
    }

    // Read full file bytes and verify SHA256
    let file_bytes = fs::read(&upload_path).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to read complete upload: {}", e))
    })?;

    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let calculated_digest = format!("sha256:{}", hex::encode(hasher.finalize()));

    if calculated_digest != expected_digest {
        fs::remove_file(&upload_path).await.ok();
        return Err(AppError::BadRequest(format!(
            "Digest mismatch: expected {}, calculated {}",
            expected_digest, calculated_digest
        )));
    }

    let size_bytes = file_bytes.len() as i64;
    let storage_path = format!("blobs/{}", calculated_digest);

    // Commit to S3 / MinIO
    state
        .s3
        .put_object()
        .bucket(&state.config.s3_bucket)
        .key(&storage_path)
        .body(ByteStream::from(file_bytes))
        .send()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("S3 upload error: {:?}", e)))?;

    // Record in database
    sqlx::query!(
        r#"
        INSERT INTO blobs (digest, size_bytes, media_type, storage_path)
        VALUES ($1, $2, 'application/octet-stream', $3)
        ON CONFLICT (digest) DO NOTHING
        "#,
        calculated_digest,
        size_bytes,
        storage_path
    )
    .execute(&state.db)
    .await?;

    // Clean up temporary buffer
    fs::remove_file(&upload_path).await.ok();

    let location = format!("/v2/{}/{}/blobs/{}", namespace, name, calculated_digest);
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
