use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransparencyLogEntry {
    pub id: Uuid,
    pub sequence_number: i64,
    pub target_type: String,
    pub target_name: String,
    pub version: String,
    pub digest: String,
    pub signature: String,
    pub public_key: String,
    pub leaf_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransparencyLogResponse {
    pub merkle_root: String,
    pub total_entries: i64,
    pub entries: Vec<TransparencyLogEntry>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyAttestationRequest {
    pub target_name: String,
    pub version: String,
    pub digest: String,
    pub signature: String,
    pub public_key: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyAttestationResponse {
    pub is_valid: bool,
    pub signer_identity: String,
    pub recorded_in_log: bool,
    pub sequence_number: Option<i64>,
    pub leaf_hash: Option<String>,
    pub merkle_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityScanDto {
    pub id: Uuid,
    pub namespace: String,
    pub name: String,
    pub tag: String,
    pub digest: String,
    pub status: String,
    pub critical_count: i32,
    pub high_count: i32,
    pub medium_count: i32,
    pub low_count: i32,
    pub cve_report: serde_json::Value,
    pub scanned_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct TriggerScanRequest {
    pub namespace: String,
    pub name: String,
    pub tag: Option<String>,
}
