use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommercialLicenseDto {
    pub id: Uuid,
    pub license_key: String,
    pub org_id: Option<Uuid>,
    pub appliance_name: String,
    pub tier: String,
    pub max_nodes: i32,
    pub max_vcpu_per_vm: i32,
    pub is_active: bool,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct IssueLicenseRequest {
    pub org_name: String,
    pub appliance_name: String,
    pub tier: Option<String>,
    pub max_nodes: Option<i32>,
    pub max_vcpu: Option<i32>,
    pub valid_days: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyLicenseRequest {
    pub license_key: String,
    pub appliance_name: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyLicenseResponse {
    pub valid: bool,
    pub license_key: String,
    pub appliance_name: String,
    pub tier: String,
    pub max_nodes: i32,
    pub max_vcpu_per_vm: i32,
    pub expires_at: DateTime<Utc>,
    pub signature_verified: bool,
}
