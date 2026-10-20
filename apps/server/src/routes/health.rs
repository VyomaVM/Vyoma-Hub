use crate::{error::AppError, state::AppState};
use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::time::Instant;

pub async fn health_check(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let start = Instant::now();

    // 1. Check PostgreSQL
    let db_start = Instant::now();
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .is_ok();
    let db_latency_ms = db_start.elapsed().as_millis();

    // 2. Check Redis
    let redis_start = Instant::now();
    let mut redis_conn = state.redis.clone();
    let redis_ok = redis::cmd("PING")
        .query_async::<String>(&mut redis_conn)
        .await
        .is_ok();
    let redis_latency_ms = redis_start.elapsed().as_millis();

    // 3. Check S3 / MinIO
    let s3_start = Instant::now();
    let s3_ok = state
        .s3
        .head_bucket()
        .bucket(&state.config.s3_bucket)
        .send()
        .await
        .is_ok();
    let s3_latency_ms = s3_start.elapsed().as_millis();

    let all_healthy = db_ok && redis_ok && s3_ok;

    Ok(Json(json!({
        "status": if all_healthy { "healthy" } else { "degraded" },
        "service": "vyoma-hub-server",
        "version": env!("CARGO_PKG_VERSION"),
        "total_latency_ms": start.elapsed().as_millis(),
        "components": {
            "postgres": { "status": if db_ok { "up" } else { "down" }, "latency_ms": db_latency_ms },
            "redis": { "status": if redis_ok { "up" } else { "down" }, "latency_ms": redis_latency_ms },
            "s3_blob_store": { "status": if s3_ok { "up" } else { "down" }, "bucket": state.config.s3_bucket, "latency_ms": s3_latency_ms }
        }
    })))
}
