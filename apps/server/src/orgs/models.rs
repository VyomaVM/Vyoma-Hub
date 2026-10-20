use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrganizationDto {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_verified: bool,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrgRequest {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMemberDto {
    pub user_id: Uuid,
    pub username: String,
    pub full_name: Option<String>,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub username: String,
    pub role: String, // 'owner', 'maintainer', 'contributor', 'viewer'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDto {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub url: String,
    pub events: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub secret: Option<String>,
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebhookDeliveryDto {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub status_code: Option<i32>,
    pub request_payload: serde_json::Value,
    pub response_body: Option<String>,
    pub delivered_at: DateTime<Utc>,
}
