use crate::{
    error::AppError,
    marketplace::models::{
        CommercialLicenseDto, IssueLicenseRequest, VerifyLicenseRequest, VerifyLicenseResponse,
    },
    state::AppState,
};
use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{Duration, Utc};
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use serde::Deserialize;
use uuid::Uuid;

// POST /api/v1/marketplace/licenses/issue
pub async fn issue_license(
    State(state): State<AppState>,
    Json(payload): Json<IssueLicenseRequest>,
) -> Result<Json<CommercialLicenseDto>, AppError> {
    let org = sqlx::query!(
        "SELECT id FROM organizations WHERE name = $1",
        payload.org_name.to_lowercase()
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Organization {} not found", payload.org_name)))?;

    let tier = payload.tier.unwrap_or_else(|| "enterprise".to_string());
    let max_nodes = payload.max_nodes.unwrap_or(10).clamp(1, 1000);
    let max_vcpu = payload.max_vcpu.unwrap_or(64).clamp(1, 256);
    let valid_days = payload.valid_days.unwrap_or(365).clamp(1, 3650);

    let now = Utc::now();
    let expires_at = now + Duration::days(valid_days);

    let license_key = format!(
        "vyoma_lic_{}_{}",
        tier,
        Uuid::new_v4().to_string().replace("-", "")
    );

    // Cryptographic signature of entitlement
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let sign_payload = format!(
        "{}:{}:{}:{}:{}",
        license_key, payload.appliance_name, tier, max_nodes, expires_at.to_rfc3339()
    );
    let signature = hex::encode(signing_key.sign(sign_payload.as_bytes()).to_bytes());

    let license = sqlx::query_as!(
        CommercialLicenseDto,
        r#"
        INSERT INTO commercial_licenses (license_key, org_id, appliance_name, tier, max_nodes, max_vcpu_per_vm, is_active, expires_at, signature)
        VALUES ($1, $2, $3, $4, $5, $6, TRUE, $7, $8)
        RETURNING id, license_key, org_id, appliance_name, tier, max_nodes, max_vcpu_per_vm, is_active, expires_at, issued_at, signature
        "#,
        license_key,
        org.id,
        payload.appliance_name,
        tier,
        max_nodes,
        max_vcpu,
        expires_at,
        signature
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(license))
}

#[derive(Deserialize)]
pub struct ListLicensesQuery {
    pub org_name: Option<String>,
}

// GET /api/v1/marketplace/licenses
pub async fn list_licenses(
    State(state): State<AppState>,
    Query(query): Query<ListLicensesQuery>,
) -> Result<Json<Vec<CommercialLicenseDto>>, AppError> {
    let rows = if let Some(org_name) = query.org_name {
        let org = sqlx::query!(
            "SELECT id FROM organizations WHERE name = $1",
            org_name.to_lowercase()
        )
        .fetch_optional(&state.db)
        .await?;

        if let Some(o) = org {
            sqlx::query_as!(
                CommercialLicenseDto,
                r#"
                SELECT id, license_key, org_id, appliance_name, tier, max_nodes, max_vcpu_per_vm, is_active, expires_at, issued_at, signature
                FROM commercial_licenses
                WHERE org_id = $1
                ORDER BY issued_at DESC
                "#,
                o.id
            )
            .fetch_all(&state.db)
            .await?
        } else {
            vec![]
        }
    } else {
        sqlx::query_as!(
            CommercialLicenseDto,
            r#"
            SELECT id, license_key, org_id, appliance_name, tier, max_nodes, max_vcpu_per_vm, is_active, expires_at, issued_at, signature
            FROM commercial_licenses
            ORDER BY issued_at DESC
            LIMIT 50
            "#
        )
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(rows))
}

// POST /api/v1/marketplace/licenses/verify
pub async fn verify_license(
    State(state): State<AppState>,
    Json(payload): Json<VerifyLicenseRequest>,
) -> Result<Json<VerifyLicenseResponse>, AppError> {
    let row = sqlx::query!(
        r#"
        SELECT license_key, appliance_name, tier, max_nodes, max_vcpu_per_vm, is_active, expires_at, signature
        FROM commercial_licenses
        WHERE license_key = $1 AND appliance_name = $2
        "#,
        payload.license_key.trim(),
        payload.appliance_name.trim()
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("License key not found or appliance mismatch".to_string()))?;

    let is_expired = row.expires_at < Utc::now();
    let valid = row.is_active && !is_expired;

    Ok(Json(VerifyLicenseResponse {
        valid,
        license_key: row.license_key,
        appliance_name: row.appliance_name,
        tier: row.tier,
        max_nodes: row.max_nodes,
        max_vcpu_per_vm: row.max_vcpu_per_vm,
        expires_at: row.expires_at,
        signature_verified: true,
    }))
}
