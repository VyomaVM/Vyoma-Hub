use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConversionJobDto {
    pub id: Uuid,
    pub source_image: String,
    pub target_appliance: String,
    pub status: String,
    pub assigned_kernel: String,
    pub rootfs_digest: Option<String>,
    pub size_bytes: i64,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitConversionRequest {
    pub source_image: String,
    pub target_appliance: Option<String>,
}
