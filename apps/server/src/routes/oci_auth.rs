use crate::{
    auth::{mint_oci_token, MaybeAuthUser, OciAccess},
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TokenQuery {
    pub service: Option<String>,
    pub scope: Option<String>,
    pub account: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Serialize)]
pub struct OciTokenResponse {
    pub token: String,
    pub access_token: String,
    pub expires_in: i64,
    pub issued_at: String,
}

// GET /v2/ (OCI API Version Check)
pub async fn oci_ping(maybe_auth: MaybeAuthUser) -> Response {
    if maybe_auth.0.is_some() {
        let mut res = (StatusCode::OK, Json(serde_json::json!({}))).into_response();
        res.headers_mut().insert(
            "Docker-Distribution-API-Version",
            HeaderValue::from_static("registry/2.0"),
        );
        res
    } else {
        // Return 401 with WWW-Authenticate challenge
        let mut res = StatusCode::UNAUTHORIZED.into_response();
        res.headers_mut().insert(
            "Docker-Distribution-API-Version",
            HeaderValue::from_static("registry/2.0"),
        );
        res.headers_mut().insert(
            header::WWW_AUTHENTICATE,
            HeaderValue::from_static(
                r#"Bearer realm="http://localhost:8080/v2/token",service="hub.vyoma.dev""#,
            ),
        );
        res
    }
}

// GET /v2/token (OCI Token Minting endpoint)
pub async fn oci_token(
    State(state): State<AppState>,
    maybe_auth: MaybeAuthUser,
    Query(query): Query<TokenQuery>,
) -> Result<Response, AppError> {
    let service = query.service.unwrap_or_else(|| "hub.vyoma.dev".to_string());
    let subject = if let Some(ref auth) = maybe_auth.0 {
        auth.username.clone()
    } else {
        query.account.unwrap_or_else(|| "anonymous".to_string())
    };

    let mut access_list = Vec::new();

    // Parse requested scope (e.g. "repository:vyoma/postgres:pull,push")
    if let Some(scope_str) = query.scope {
        let parts: Vec<&str> = scope_str.split(':').collect();
        if parts.len() >= 3 {
            let resource_type = parts[0]; // "repository"
            let resource_name = parts[1]; // "vyoma/postgres"
            let actions: Vec<String> = parts[2].split(',').map(|s| s.trim().to_string()).collect();

            // Permission check:
            // If authenticated and owner/member, grant pull/push
            // If public repo, grant pull
            // If not authenticated, grant pull only if public
            let mut granted_actions = Vec::new();
            let repo_parts: Vec<&str> = resource_name.splitn(2, '/').collect();
            let (namespace, name) = if repo_parts.len() == 2 {
                (repo_parts[0], repo_parts[1])
            } else {
                ("library", repo_parts[0])
            };

            let repo = sqlx::query!(
                "SELECT is_public FROM repositories WHERE namespace = $1 AND name = $2",
                namespace,
                name
            )
            .fetch_optional(&state.db)
            .await?;

            let is_public = repo.map(|r| r.is_public).unwrap_or(true); // Default to public for new pushes or known images

            for action in actions {
                if action == "pull" && is_public {
                    granted_actions.push(action);
                } else if let Some(ref auth) = maybe_auth.0 {
                    if action == "pull" || action == "push" {
                        if auth.username == namespace || auth.is_admin {
                            granted_actions.push(action);
                        } else {
                            // Check org membership
                            let is_member = sqlx::query!(
                                r#"
                                SELECT 1 as exists FROM org_members om
                                JOIN organizations o ON o.id = om.org_id
                                WHERE o.name = $1 AND om.user_id = $2
                                "#,
                                namespace,
                                auth.id
                            )
                            .fetch_optional(&state.db)
                            .await?
                            .is_some();

                            if is_member {
                                granted_actions.push(action);
                            }
                        }
                    }
                }
            }

            access_list.push(OciAccess {
                r#type: resource_type.to_string(),
                name: resource_name.to_string(),
                actions: granted_actions,
            });
        }
    }

    let expiry_minutes = 60;
    let token = mint_oci_token(
        &subject,
        access_list,
        &state.config.jwt_secret,
        &state.config.token_issuer,
        &service,
        expiry_minutes,
    )?;

    let now_iso = Utc::now().to_rfc3339();
    let body = OciTokenResponse {
        token: token.clone(),
        access_token: token,
        expires_in: expiry_minutes * 60,
        issued_at: now_iso,
    };

    let mut res = (StatusCode::OK, Json(body)).into_response();
    res.headers_mut().insert(
        "Docker-Distribution-API-Version",
        HeaderValue::from_static("registry/2.0"),
    );
    Ok(res)
}
