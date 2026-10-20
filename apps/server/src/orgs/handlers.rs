use crate::{
    auth::extractor::{AuthUser, MaybeAuthUser},
    error::AppError,
    orgs::models::{
        AddMemberRequest, CreateOrgRequest, CreateWebhookRequest, OrgMemberDto, OrganizationDto,
        WebhookDeliveryDto, WebhookDto,
    },
    state::AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

// POST /api/v1/orgs
pub async fn create_organization(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Json<OrganizationDto>, AppError> {
    let name = payload.name.trim().to_lowercase();
    if name.len() < 3 || name.len() > 64 {
        return Err(AppError::BadRequest("Organization name must be 3-64 characters".into()));
    }

    let mut tx = state.db.begin().await?;

    let org_row = sqlx::query!(
        r#"
        INSERT INTO organizations (name, display_name, description)
        VALUES ($1, $2, $3)
        RETURNING id, name, display_name, description, is_verified, avatar_url, created_at
        "#,
        name,
        payload.display_name.trim(),
        payload.description.as_deref()
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("An organization with this name already exists".into())
        }
        _ => AppError::Database(e),
    })?;

    // Add creator as owner
    sqlx::query!(
        r#"
        INSERT INTO org_members (org_id, user_id, role)
        VALUES ($1, $2, 'owner')
        "#,
        org_row.id,
        auth.id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(OrganizationDto {
        id: org_row.id,
        name: org_row.name,
        display_name: org_row.display_name,
        description: org_row.description,
        is_verified: org_row.is_verified,
        avatar_url: org_row.avatar_url,
        role: Some("owner".to_string()),
        created_at: org_row.created_at,
    }))
}

// GET /api/v1/orgs
pub async fn list_organizations(
    State(state): State<AppState>,
    MaybeAuthUser(auth): MaybeAuthUser,
) -> Result<Json<Vec<OrganizationDto>>, AppError> {
    let orgs = if let Some(auth_user) = auth {
        sqlx::query_as!(
            OrganizationDto,
            r#"
            SELECT o.id, o.name, o.display_name, o.description, o.is_verified, o.avatar_url,
                   m.role, o.created_at
            FROM organizations o
            LEFT JOIN org_members m ON m.org_id = o.id AND m.user_id = $1
            ORDER BY o.is_verified DESC, o.created_at DESC
            LIMIT 50
            "#,
            auth_user.id
        )
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as!(
            OrganizationDto,
            r#"
            SELECT id, name, display_name, description, is_verified, avatar_url,
                   NULL as role, created_at
            FROM organizations
            ORDER BY is_verified DESC, created_at DESC
            LIMIT 50
            "#
        )
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(orgs))
}

// GET /api/v1/orgs/:name
pub async fn get_organization(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<OrganizationDto>, AppError> {
    let org = sqlx::query_as!(
        OrganizationDto,
        r#"
        SELECT id, name, display_name, description, is_verified, avatar_url,
               NULL as role, created_at
        FROM organizations
        WHERE name = $1
        "#,
        name.to_lowercase()
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Organization {} not found", name)))?;

    Ok(Json(org))
}

// GET /api/v1/orgs/:name/members
pub async fn list_members(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<OrgMemberDto>>, AppError> {
    let org = sqlx::query!("SELECT id FROM organizations WHERE name = $1", name.to_lowercase())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Organization {} not found", name)))?;

    let members = sqlx::query_as!(
        OrgMemberDto,
        r#"
        SELECT u.id as user_id, u.username, u.full_name, m.role, m.created_at as joined_at
        FROM org_members m
        JOIN users u ON u.id = m.user_id
        WHERE m.org_id = $1
        ORDER BY m.created_at ASC
        "#,
        org.id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(members))
}

// POST /api/v1/orgs/:name/members
pub async fn add_member(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(name): Path<String>,
    Json(payload): Json<AddMemberRequest>,
) -> Result<Json<OrgMemberDto>, AppError> {
    let org = sqlx::query!("SELECT id FROM organizations WHERE name = $1", name.to_lowercase())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Organization {} not found", name)))?;

    let user = sqlx::query!("SELECT id, username, full_name FROM users WHERE username = $1", payload.username.trim())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User {} not found", payload.username)))?;

    let role = match payload.role.as_str() {
        "owner" | "admin" | "member" | "viewer" => payload.role,
        _ => "member".to_string(),
    };

    sqlx::query!(
        r#"
        INSERT INTO org_members (org_id, user_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (org_id, user_id) DO UPDATE SET role = EXCLUDED.role
        "#,
        org.id,
        user.id,
        role
    )
    .execute(&state.db)
    .await?;

    Ok(Json(OrgMemberDto {
        user_id: user.id,
        username: user.username,
        full_name: user.full_name,
        role,
        joined_at: chrono::Utc::now(),
    }))
}

// POST /api/v1/orgs/:name/webhooks
pub async fn create_webhook(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(name): Path<String>,
    Json(payload): Json<CreateWebhookRequest>,
) -> Result<Json<WebhookDto>, AppError> {
    let org = sqlx::query!("SELECT id FROM organizations WHERE name = $1", name.to_lowercase())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Organization {} not found", name)))?;

    let secret = payload.secret.unwrap_or_else(|| format!("whsec_{:x}", md5_or_uuid()));
    let events = payload.events.unwrap_or_else(|| vec!["push".to_string(), "release".to_string()]);

    let row = sqlx::query!(
        r#"
        INSERT INTO webhooks (org_id, url, secret, events, is_active)
        VALUES ($1, $2, $3, $4, TRUE)
        RETURNING id, org_id, url, events, is_active, created_at
        "#,
        org.id,
        payload.url,
        secret,
        &events
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(WebhookDto {
        id: row.id,
        org_id: row.org_id,
        url: row.url,
        events: row.events,
        is_active: row.is_active,
        created_at: row.created_at,
    }))
}

// GET /api/v1/orgs/:name/webhooks
pub async fn list_webhooks(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<WebhookDto>>, AppError> {
    let org = sqlx::query!("SELECT id FROM organizations WHERE name = $1", name.to_lowercase())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Organization {} not found", name)))?;

    let rows = sqlx::query!(
        r#"
        SELECT id, org_id, url, events, is_active, created_at
        FROM webhooks
        WHERE org_id = $1
        ORDER BY created_at DESC
        "#,
        org.id
    )
    .fetch_all(&state.db)
    .await?;

    let list = rows
        .into_iter()
        .map(|r| WebhookDto {
            id: r.id,
            org_id: r.org_id,
            url: r.url,
            events: r.events,
            is_active: r.is_active,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(list))
}

// POST /api/v1/webhooks/:id/test
pub async fn test_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<Uuid>,
) -> Result<Json<WebhookDeliveryDto>, AppError> {
    let hook = sqlx::query!(
        "SELECT id, url, secret FROM webhooks WHERE id = $1",
        webhook_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Webhook not found".into()))?;

    let payload = json!({
        "event": "ping",
        "webhook_id": hook.id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "message": "Vyoma Hub test event ping"
    });

    // Sign payload with HMAC-SHA256
    let mut mac = HmacSha256::new_from_slice(hook.secret.as_bytes())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("HMAC initialization failed: {}", e)))?;
    mac.update(payload.to_string().as_bytes());
    let _signature = hex::encode(mac.finalize().into_bytes());

    let delivery_id = Uuid::new_v4();
    let status_code = 200; // Simulated test dispatch success

    sqlx::query!(
        r#"
        INSERT INTO webhook_deliveries (id, webhook_id, event_type, status_code, request_payload, response_body)
        VALUES ($1, $2, 'ping', $3, $4, '{"ok": true}')
        "#,
        delivery_id,
        hook.id,
        status_code,
        payload
    )
    .execute(&state.db)
    .await?;

    Ok(Json(WebhookDeliveryDto {
        id: delivery_id,
        webhook_id: hook.id,
        event_type: "ping".to_string(),
        status_code: Some(status_code),
        request_payload: payload,
        response_body: Some(r#"{"ok": true}"#.to_string()),
        delivered_at: chrono::Utc::now(),
    }))
}

fn md5_or_uuid() -> Uuid {
    Uuid::new_v4()
}
