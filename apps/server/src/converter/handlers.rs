use crate::{
    converter::{
        models::{ConversionJobDto, SubmitConversionRequest},
        worker::execute_conversion,
    },
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

// POST /api/v1/converter/jobs
pub async fn submit_conversion_job(
    State(state): State<AppState>,
    Json(payload): Json<SubmitConversionRequest>,
) -> Result<Json<ConversionJobDto>, AppError> {
    let source_image = payload.source_image.trim().to_string();
    if source_image.is_empty() {
        return Err(AppError::BadRequest("Source image name is required".into()));
    }

    let target_appliance = payload.target_appliance.unwrap_or_else(|| {
        let clean_name = source_image
            .split('/')
            .last()
            .unwrap_or(&source_image)
            .split(':')
            .next()
            .unwrap_or("app");
        format!("vyoma/{}", clean_name)
    });

    let job_id = Uuid::new_v4();

    let job = sqlx::query_as!(
        ConversionJobDto,
        r#"
        INSERT INTO conversion_jobs (id, source_image, target_appliance, status, assigned_kernel)
        VALUES ($1, $2, $3, 'queued', 'evaluating')
        RETURNING id, source_image, target_appliance, status, assigned_kernel, rootfs_digest, size_bytes, error_message, created_at, completed_at
        "#,
        job_id,
        source_image,
        target_appliance
    )
    .fetch_one(&state.db)
    .await?;

    // Spawn background worker execution asynchronously
    let state_clone = state.clone();
    let src = source_image.clone();
    let tgt = target_appliance.clone();
    tokio::spawn(async move {
        if let Err(e) = execute_conversion(&state_clone, job_id, &src, &tgt).await {
            tracing::error!("Conversion job {} failed: {:?}", job_id, e);
            let _ = sqlx::query!(
                "UPDATE conversion_jobs SET status = 'failed', error_message = $1 WHERE id = $2",
                e.to_string(),
                job_id
            )
            .execute(&state_clone.db)
            .await;
        }
    });

    Ok(Json(job))
}

// GET /api/v1/converter/jobs
pub async fn list_conversion_jobs(
    State(state): State<AppState>,
) -> Result<Json<Vec<ConversionJobDto>>, AppError> {
    let jobs = sqlx::query_as!(
        ConversionJobDto,
        r#"
        SELECT id, source_image, target_appliance, status, assigned_kernel, rootfs_digest, size_bytes, error_message, created_at, completed_at
        FROM conversion_jobs
        ORDER BY created_at DESC
        LIMIT 25
        "#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(jobs))
}

// GET /api/v1/converter/jobs/:id
pub async fn get_conversion_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<ConversionJobDto>, AppError> {
    let job = sqlx::query_as!(
        ConversionJobDto,
        r#"
        SELECT id, source_image, target_appliance, status, assigned_kernel, rootfs_digest, size_bytes, error_message, created_at, completed_at
        FROM conversion_jobs
        WHERE id = $1
        "#,
        job_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Conversion job {} not found", job_id)))?;

    Ok(Json(job))
}
