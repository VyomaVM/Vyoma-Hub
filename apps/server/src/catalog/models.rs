use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelProfileDto {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub workload_type: String,
    pub vmlinuz_digest: String,
    pub config_digest: Option<String>,
    pub ed25519_signature: String,
    pub ed25519_public_key: String,
    pub is_official: bool,
    pub description: String,
    pub recommended_cmdline: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    pub id: Uuid,
    pub namespace: String,
    pub name: String,
    pub description: Option<String>,
    pub pull_count: i64,
    pub star_count: i32,
    pub is_verified: bool,
    pub tags: Vec<String>,
    pub recommended_kernel: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryDetail {
    pub id: Uuid,
    pub namespace: String,
    pub name: String,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub pull_count: i64,
    pub star_count: i32,
    pub is_public: bool,
    pub is_verified: bool,
    pub tags: Vec<String>,
    pub recommended_kernel: Option<KernelProfileDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CatalogQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub sort: Option<String>, // "popular", "stars", "recent"
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmifHardware {
    pub vcpu: i32,
    pub memory_mb: i32,
    pub disk_gb: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmifManifest {
    pub schema_version: u32,
    pub appliance: String,
    pub version: String,
    pub recommended_kernel: String,
    pub boot_args: String,
    pub hardware: VmifHardware,
    pub rootfs_digest: String,
    pub signature: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UploadApplianceRequest {
    pub namespace: String,
    pub name: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub recommended_kernel: Option<String>,
    pub boot_args: Option<String>,
    pub vcpu: Option<i32>,
    pub memory_mb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub rootfs_base64: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UploadApplianceResponse {
    pub status: String,
    pub repository: String,
    pub tag: String,
    pub digest: String,
    pub manifest: VmifManifest,
}

