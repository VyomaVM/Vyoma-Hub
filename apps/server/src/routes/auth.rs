use crate::{
    auth::{
        generate_personal_access_token, hash_password, mint_user_token, verify_password,
        AuthUser,
    },
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String, // username or email
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    pub scopes: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub id: Uuid,
    pub name: String,
    pub token: String, // Plaintext raw token, shown once
    pub prefix: String,
    pub scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct TokenInfo {
    pub id: Uuid,
    pub name: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let username = payload.username.trim().to_lowercase();
    let email = payload.email.trim().to_lowercase();

    if username.len() < 3 || username.len() > 32 {
        return Err(AppError::BadRequest(
            "Username must be between 3 and 32 characters".to_string(),
        ));
    }
    if !username.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(AppError::BadRequest(
            "Username can only contain alphanumeric characters, hyphens, and underscores".to_string(),
        ));
    }
    if payload.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    let password_hash = hash_password(&payload.password)?;

    let user_row = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash, full_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, full_name, avatar_url, is_admin, created_at
        "#,
        username,
        email,
        password_hash,
        payload.full_name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::Conflict("Username or email already exists".to_string());
            }
        }
        AppError::Database(e)
    })?;

    let token = mint_user_token(
        user_row.id,
        &user_row.username,
        user_row.is_admin,
        &state.config.jwt_secret,
        &state.config.token_issuer,
        state.config.token_expiry_hours,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: UserProfile {
            id: user_row.id,
            username: user_row.username,
            email: user_row.email,
            full_name: user_row.full_name,
            avatar_url: user_row.avatar_url,
            is_admin: user_row.is_admin,
            created_at: user_row.created_at,
        },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let login_input = payload.login.trim().to_lowercase();

    let user_row = sqlx::query!(
        r#"
        SELECT id, username, email, password_hash, full_name, avatar_url, is_admin, created_at
        FROM users
        WHERE username = $1 OR email = $1
        "#,
        login_input
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username or password".to_string()))?;

    let password_valid = match user_row.password_hash {
        Some(ref hash) => verify_password(&payload.password, hash),
        None => false,
    };
    if !password_valid {
        return Err(AppError::Unauthorized("Invalid username or password".to_string()));
    }

    let token = mint_user_token(
        user_row.id,
        &user_row.username,
        user_row.is_admin,
        &state.config.jwt_secret,
        &state.config.token_issuer,
        state.config.token_expiry_hours,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: UserProfile {
            id: user_row.id,
            username: user_row.username,
            email: user_row.email,
            full_name: user_row.full_name,
            avatar_url: user_row.avatar_url,
            is_admin: user_row.is_admin,
            created_at: user_row.created_at,
        },
    }))
}

pub async fn me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<UserProfile>, AppError> {
    let user_row = sqlx::query!(
        r#"
        SELECT id, username, email, full_name, avatar_url, is_admin, created_at
        FROM users
        WHERE id = $1
        "#,
        auth.id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(UserProfile {
        id: user_row.id,
        username: user_row.username,
        email: user_row.email,
        full_name: user_row.full_name,
        avatar_url: user_row.avatar_url,
        is_admin: user_row.is_admin,
        created_at: user_row.created_at,
    }))
}

pub async fn create_token(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, AppError> {
    let token_gen = generate_personal_access_token();
    let scopes = payload.scopes.unwrap_or_else(|| vec!["read".to_string(), "write".to_string()]);

    let row = sqlx::query!(
        r#"
        INSERT INTO api_tokens (user_id, name, token_hash, token_prefix, scopes)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, token_prefix, scopes, created_at
        "#,
        auth.id,
        payload.name,
        token_gen.token_hash,
        token_gen.prefix,
        &scopes
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(CreateTokenResponse {
        id: row.id,
        name: row.name,
        token: token_gen.raw_token,
        prefix: row.token_prefix,
        scopes: row.scopes,
        created_at: row.created_at,
    }))
}

pub async fn list_tokens(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<TokenInfo>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, token_prefix, scopes, last_used_at, created_at
        FROM api_tokens
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        auth.id
    )
    .fetch_all(&state.db)
    .await?;

    let tokens = rows
        .into_iter()
        .map(|r| TokenInfo {
            id: r.id,
            name: r.name,
            prefix: r.token_prefix,
            scopes: r.scopes,
            last_used_at: r.last_used_at,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(tokens))
}

pub async fn delete_token(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(token_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query!(
        "DELETE FROM api_tokens WHERE id = $1 AND user_id = $2",
        token_id,
        auth.id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Token not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

#[derive(Deserialize)]
pub struct GitHubCallbackRequest {
    pub code: String,
    pub redirect_uri: Option<String>,
}

#[derive(Serialize)]
pub struct GitHubAuthUrlResponse {
    pub url: String,
    pub client_id: String,
}

pub async fn github_auth_url(
    State(_state): State<AppState>,
) -> Result<Json<GitHubAuthUrlResponse>, AppError> {
    let client_id = std::env::var("GITHUB_CLIENT_ID")
        .unwrap_or_else(|_| "vyoma-hub-dev-client-id".to_string());
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&scope=user:email,read:user",
        client_id
    );

    Ok(Json(GitHubAuthUrlResponse { url, client_id }))
}

pub async fn github_callback(
    State(state): State<AppState>,
    Json(payload): Json<GitHubCallbackRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let code = payload.code.trim();
    if code.is_empty() {
        return Err(AppError::BadRequest("Authorization code required".into()));
    }

    let code_slug = if code.len() > 8 { &code[..8] } else { code };
    let gh_username = format!("gh-{}", code_slug);
    let gh_email = format!("{}@users.noreply.github.com", gh_username);
    let gh_id = format!("github_{}", code);

    let mut tx = state.db.begin().await?;

    let existing_oauth = sqlx::query!(
        "SELECT user_id FROM user_oauth_accounts WHERE provider = 'github' AND provider_user_id = $1",
        gh_id
    )
    .fetch_optional(&mut *tx)
    .await?;

    let user_id = if let Some(oauth) = existing_oauth {
        oauth.user_id
    } else {
        let user = sqlx::query!(
            r#"
            INSERT INTO users (username, email, full_name, avatar_url, is_admin)
            VALUES ($1, $2, $3, 'https://github.com/identicons/app.png', FALSE)
            ON CONFLICT (username) DO UPDATE SET avatar_url = EXCLUDED.avatar_url
            RETURNING id
            "#,
            gh_username,
            gh_email,
            format!("GitHub User ({})", gh_username)
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO user_oauth_accounts (user_id, provider, provider_user_id, profile_data)
            VALUES ($1, 'github', $2, $3)
            ON CONFLICT (provider, provider_user_id) DO NOTHING
            "#,
            user.id,
            gh_id,
            serde_json::json!({ "code": code, "provider": "github" })
        )
        .execute(&mut *tx)
        .await?;

        user.id
    };

    let user_row = sqlx::query!(
        r#"
        SELECT id, username, email, full_name, avatar_url, is_admin, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    let token = mint_user_token(
        user_row.id,
        &user_row.username,
        user_row.is_admin,
        &state.config.jwt_secret,
        &state.config.token_issuer,
        state.config.token_expiry_hours,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: UserProfile {
            id: user_row.id,
            username: user_row.username,
            email: user_row.email,
            full_name: user_row.full_name,
            avatar_url: user_row.avatar_url,
            is_admin: user_row.is_admin,
            created_at: user_row.created_at,
        },
    }))
}

