use crate::{
    auth::{jwt::verify_user_token, password::verify_password, token::hash_token},
    error::AppError,
    state::AppState,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let maybe_auth = resolve_auth(parts, &app_state).await?;
        maybe_auth.ok_or_else(|| AppError::Unauthorized("Authentication required".to_string()))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MaybeAuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let maybe_auth = resolve_auth(parts, &app_state).await?;
        Ok(MaybeAuthUser(maybe_auth))
    }
}

async fn resolve_auth(parts: &Parts, state: &AppState) -> Result<Option<AuthUser>, AppError> {
    let auth_header = match parts.headers.get(header::AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Ok(s) => s,
            Err(_) => return Ok(None),
        },
        None => return Ok(None),
    };

    // 1. Bearer Token (JWT or PAT)
    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        let token = token.trim();

        // Check if it's a Personal Access Token (starts with vyh_)
        if token.starts_with("vyh_") {
            let token_hash = hash_token(token);
            let row = sqlx::query!(
                r#"
                SELECT u.id, u.username, u.is_admin
                FROM api_tokens t
                JOIN users u ON u.id = t.user_id
                WHERE t.token_hash = $1
                  AND (t.expires_at IS NULL OR t.expires_at > NOW())
                "#,
                token_hash
            )
            .fetch_optional(&state.db)
            .await?;

            if let Some(user) = row {
                // Update last used timestamp in background
                let token_hash_clone = token_hash.clone();
                let pool = state.db.clone();
                tokio::spawn(async move {
                    sqlx::query!(
                        "UPDATE api_tokens SET last_used_at = NOW() WHERE token_hash = $1",
                        token_hash_clone
                    )
                    .execute(&pool)
                    .await
                    .ok();
                });

                return Ok(Some(AuthUser {
                    id: user.id,
                    username: user.username,
                    is_admin: user.is_admin,
                }));
            }
            return Ok(None);
        }

        // Standard JWT token verification
        match verify_user_token(token, &state.config.jwt_secret) {
            Ok(claims) => {
                if let Ok(user_uuid) = Uuid::parse_str(&claims.sub) {
                    return Ok(Some(AuthUser {
                        id: user_uuid,
                        username: claims.username,
                        is_admin: claims.is_admin,
                    }));
                }
            }
            Err(_) => return Ok(None),
        }
    }

    // 2. Basic Auth (username:password or username:PAT)
    if let Some(basic) = auth_header.strip_prefix("Basic ") {
        use base64::Engine;
        let basic = basic.trim();
        if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(basic) {
            if let Ok(cred_str) = String::from_utf8(decoded) {
                let mut parts = cred_str.splitn(2, ':');
                if let (Some(username), Some(secret)) = (parts.next(), parts.next()) {
                    // Check if secret is a PAT
                    if secret.starts_with("vyh_") {
                        let token_hash = hash_token(secret);
                        let row = sqlx::query!(
                            r#"
                            SELECT u.id, u.username, u.is_admin
                            FROM api_tokens t
                            JOIN users u ON u.id = t.user_id
                            WHERE u.username = $1 AND t.token_hash = $2
                              AND (t.expires_at IS NULL OR t.expires_at > NOW())
                            "#,
                            username,
                            token_hash
                        )
                        .fetch_optional(&state.db)
                        .await?;

                        if let Some(user) = row {
                            return Ok(Some(AuthUser {
                                id: user.id,
                                username: user.username,
                                is_admin: user.is_admin,
                            }));
                        }
                    }

                    // Otherwise verify against password
                    let user_row = sqlx::query!(
                        "SELECT id, username, password_hash, is_admin FROM users WHERE username = $1",
                        username
                    )
                    .fetch_optional(&state.db)
                    .await?;

                    if let Some(user) = user_row {
                        if let Some(ref hash) = user.password_hash {
                            if verify_password(secret, hash) {
                                return Ok(Some(AuthUser {
                                    id: user.id,
                                    username: user.username,
                                    is_admin: user.is_admin,
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}
