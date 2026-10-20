use crate::{
    catalog::models::{CatalogItem, CatalogQuery, KernelProfileDto, RepositoryDetail},
    error::AppError,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct RepoCatalogRow {
    id: Uuid,
    namespace: String,
    name: String,
    description: Option<String>,
    pull_count: i64,
    star_count: i32,
    created_at: DateTime<Utc>,
    is_verified: bool,
}

// GET /api/v1/catalog
pub async fn list_catalog(
    State(state): State<AppState>,
    Query(query): Query<CatalogQuery>,
) -> Result<Json<Vec<CatalogItem>>, AppError> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let page = query.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let rows: Vec<RepoCatalogRow> = if let Some(pattern) = query.q {
        let pattern = format!("%{}%", pattern.trim().to_lowercase());
        sqlx::query_as!(
            RepoCatalogRow,
            r#"
            SELECT r.id, r.namespace, r.name, r.description, r.pull_count, r.star_count,
                   r.created_at, COALESCE(o.is_verified, false) as "is_verified!"
            FROM repositories r
            LEFT JOIN organizations o ON o.name = r.namespace
            WHERE r.is_public = TRUE
              AND (LOWER(r.name) LIKE $1 OR LOWER(r.namespace) LIKE $1 OR LOWER(COALESCE(r.description, '')) LIKE $1)
            ORDER BY r.pull_count DESC, r.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            pattern,
            limit,
            offset
        )
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as!(
            RepoCatalogRow,
            r#"
            SELECT r.id, r.namespace, r.name, r.description, r.pull_count, r.star_count,
                   r.created_at, COALESCE(o.is_verified, false) as "is_verified!"
            FROM repositories r
            LEFT JOIN organizations o ON o.name = r.namespace
            WHERE r.is_public = TRUE
            ORDER BY r.pull_count DESC, r.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&state.db)
        .await?
    };

    let mut catalog = Vec::new();
    for r in rows {
        // Fetch tags for this repo
        let tag_rows = sqlx::query!(
            "SELECT name FROM tags WHERE repository_id = $1 ORDER BY updated_at DESC LIMIT 5",
            r.id
        )
        .fetch_all(&state.db)
        .await?;

        let tags = tag_rows.into_iter().map(|t| t.name).collect();

        // Recommend kernel based on appliance name
        let recommended_kernel = match r.name.as_str() {
            "postgres" | "postgresql" | "mongodb" | "mysql" | "clickhouse" => Some("database".to_string()),
            "nginx" | "envoy" | "traefik" | "haproxy" => Some("edge-network".to_string()),
            "pytorch" | "ollama" | "tensorrt" => Some("ai-inference".to_string()),
            "vault" | "auth-gateway" | "vpn" => Some("hardened-security".to_string()),
            _ => Some("universal-general".to_string()),
        };

        catalog.push(CatalogItem {
            id: r.id,
            namespace: r.namespace,
            name: r.name,
            description: r.description,
            pull_count: r.pull_count,
            star_count: r.star_count,
            is_verified: r.is_verified,
            tags,
            recommended_kernel,
            created_at: r.created_at,
        });
    }

    Ok(Json(catalog))
}

// GET /api/v1/repos/:namespace/:name
pub async fn get_repo(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
) -> Result<Json<RepositoryDetail>, AppError> {
    let repo_row = sqlx::query!(
        r#"
        SELECT r.id, r.namespace, r.name, r.description, r.readme, r.is_public,
               r.pull_count, r.star_count, r.created_at, r.updated_at,
               COALESCE(o.is_verified, false) as "is_verified!"
        FROM repositories r
        LEFT JOIN organizations o ON o.name = r.namespace
        WHERE r.namespace = $1 AND r.name = $2
        "#,
        namespace,
        name
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Repository not found".to_string()))?;

    let tag_rows = sqlx::query!(
        "SELECT name FROM tags WHERE repository_id = $1 ORDER BY updated_at DESC",
        repo_row.id
    )
    .fetch_all(&state.db)
    .await?;

    let tags = tag_rows.into_iter().map(|t| t.name).collect();

    // Determine recommended kernel
    let kernel_name = match repo_row.name.as_str() {
        "postgres" | "postgresql" | "mongodb" | "mysql" | "clickhouse" => "database",
        "nginx" | "envoy" | "traefik" => "edge-network",
        "pytorch" | "ollama" => "ai-inference",
        "vault" => "hardened-security",
        _ => "universal-general",
    };

    let kernel_row = sqlx::query!(
        r#"
        SELECT id, name, version, workload_type, vmlinuz_digest, config_digest,
               ed25519_signature, ed25519_public_key, is_official, description,
               recommended_cmdline, created_at
        FROM kernel_profiles
        WHERE name = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        kernel_name
    )
    .fetch_optional(&state.db)
    .await?;

    let recommended_kernel = kernel_row.map(|k| KernelProfileDto {
        id: k.id,
        name: k.name,
        version: k.version,
        workload_type: k.workload_type,
        vmlinuz_digest: k.vmlinuz_digest,
        config_digest: k.config_digest,
        ed25519_signature: k.ed25519_signature,
        ed25519_public_key: k.ed25519_public_key,
        is_official: k.is_official,
        description: k.description,
        recommended_cmdline: k.recommended_cmdline,
        created_at: k.created_at,
    });

    Ok(Json(RepositoryDetail {
        id: repo_row.id,
        namespace: repo_row.namespace,
        name: repo_row.name,
        description: repo_row.description,
        readme: repo_row.readme,
        pull_count: repo_row.pull_count,
        star_count: repo_row.star_count,
        is_public: repo_row.is_public,
        is_verified: repo_row.is_verified,
        tags,
        recommended_kernel,
        created_at: repo_row.created_at,
        updated_at: repo_row.updated_at,
    }))
}

// GET /api/v1/kernels
pub async fn list_kernels(
    State(state): State<AppState>,
) -> Result<Json<Vec<KernelProfileDto>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, version, workload_type, vmlinuz_digest, config_digest,
               ed25519_signature, ed25519_public_key, is_official, description,
               recommended_cmdline, created_at
        FROM kernel_profiles
        ORDER BY is_official DESC, workload_type ASC
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let profiles = rows
        .into_iter()
        .map(|k| KernelProfileDto {
            id: k.id,
            name: k.name,
            version: k.version,
            workload_type: k.workload_type,
            vmlinuz_digest: k.vmlinuz_digest,
            config_digest: k.config_digest,
            ed25519_signature: k.ed25519_signature,
            ed25519_public_key: k.ed25519_public_key,
            is_official: k.is_official,
            description: k.description,
            recommended_cmdline: k.recommended_cmdline,
            created_at: k.created_at,
        })
        .collect();

    Ok(Json(profiles))
}

// GET /api/v1/kernels/:name/:version
pub async fn get_kernel(
    State(state): State<AppState>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<KernelProfileDto>, AppError> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, version, workload_type, vmlinuz_digest, config_digest,
               ed25519_signature, ed25519_public_key, is_official, description,
               recommended_cmdline, created_at
        FROM kernel_profiles
        WHERE name = $1 AND version = $2
        "#,
        name,
        version
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Kernel profile not found".to_string()))?;

    Ok(Json(KernelProfileDto {
        id: row.id,
        name: row.name,
        version: row.version,
        workload_type: row.workload_type,
        vmlinuz_digest: row.vmlinuz_digest,
        config_digest: row.config_digest,
        ed25519_signature: row.ed25519_signature,
        ed25519_public_key: row.ed25519_public_key,
        is_official: row.is_official,
        description: row.description,
        recommended_cmdline: row.recommended_cmdline,
        created_at: row.created_at,
    }))
}
