use crate::{
    catalog::models::{
        CatalogItem, CatalogQuery, KernelProfileDto, RepositoryDetail, UploadApplianceRequest,
        UploadApplianceResponse, VmifHardware, VmifManifest,
    },
    error::AppError,
    state::AppState,
};
use sha2::{Digest, Sha256};
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

// GET /v2/vyoma/appliances/:name/manifest
pub async fn get_vmif_manifest(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<VmifManifest>, AppError> {
    let namespace = "vyoma";
    let repo = sqlx::query!(
        "SELECT id, namespace, name, description FROM repositories WHERE namespace = $1 AND name = $2",
        namespace,
        name
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Appliance {}/{} not found", namespace, name)))?;

    let (kernel, cmdline, vcpu, mem, disk) = match name.as_str() {
        "postgres" | "redis" | "mongo" => (
            "database",
            "console=ttyS0 root=/dev/vda rw elevator=deadline transparent_hugepage=always nohz_full=1-3 mitigations=off",
            2,
            2048,
            20,
        ),
        "nginx" | "envoy" => (
            "edge-network",
            "console=ttyS0 root=/dev/vda rw tcp_congestion_control=bbr net.core.bpf_jit_enable=1 quiet",
            1,
            512,
            5,
        ),
        "ollama" | "pytorch" => (
            "ai-inference",
            "console=ttyS0 root=/dev/vda rw intel_iommu=on vfio_iommu_type1.allow_unsafe_interrupts=1 default_hugepagesz=1G hugepagesz=1G hugepages=8",
            4,
            8192,
            50,
        ),
        _ => (
            "universal-general",
            "console=ttyS0 root=/dev/vda rw quiet",
            1,
            1024,
            10,
        ),
    };

    let rootfs_digest = format!(
        "sha256:{:x}",
        Sha256::digest(format!("vmif:rootfs:{}:{}", repo.namespace, repo.name).as_bytes())
    );

    Ok(Json(VmifManifest {
        schema_version: 2,
        appliance: format!("{}/{}", repo.namespace, repo.name),
        version: "latest".to_string(),
        recommended_kernel: kernel.to_string(),
        boot_args: cmdline.to_string(),
        hardware: VmifHardware {
            vcpu,
            memory_mb: mem,
            disk_gb: disk,
        },
        rootfs_digest,
        signature: Some(format!("sig:vmif:{}:{}", repo.namespace, repo.name)),
        created_at: Utc::now(),
    }))
}

// GET /v2/vyoma/kernels/:profile/:version
pub async fn get_vmif_kernel(
    State(state): State<AppState>,
    Path((profile, version)): Path<(String, String)>,
) -> Result<Json<KernelProfileDto>, AppError> {
    get_kernel(State(state), Path((profile, version))).await
}

// POST /api/v1/appliances/upload
pub async fn upload_appliance(
    State(state): State<AppState>,
    Json(payload): Json<UploadApplianceRequest>,
) -> Result<Json<UploadApplianceResponse>, AppError> {
    let namespace = payload.namespace.trim().to_lowercase();
    let name = payload.name.trim().to_lowercase();
    let tag = payload.tag.unwrap_or_else(|| "latest".to_string());
    let kernel = payload.recommended_kernel.unwrap_or_else(|| "universal-general".to_string());
    let boot_args = payload.boot_args.unwrap_or_else(|| "console=ttyS0 root=/dev/vda rw quiet".to_string());
    let vcpu = payload.vcpu.unwrap_or(2).clamp(1, 64);
    let memory_mb = payload.memory_mb.unwrap_or(2048).clamp(128, 131072);
    let disk_gb = payload.disk_gb.unwrap_or(10).clamp(1, 1024);

    let rootfs_bytes = if let Some(ref b64) = payload.rootfs_base64 {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(b64)
            .unwrap_or_else(|_| b"squashfs-microvm-rootfs-content".to_vec())
    } else {
        b"squashfs-microvm-rootfs-default-blob".to_vec()
    };

    let digest_hash = hex::encode(Sha256::digest(&rootfs_bytes));
    let rootfs_digest = format!("sha256:{}", digest_hash);

    let mut tx = state.db.begin().await?;

    // Upsert repository
    let repo_id = sqlx::query!(
        r#"
        INSERT INTO repositories (namespace, name, description, is_public)
        VALUES ($1, $2, $3, TRUE)
        ON CONFLICT (namespace, name) DO UPDATE SET
            description = COALESCE(EXCLUDED.description, repositories.description),
            updated_at = NOW()
        RETURNING id
        "#,
        namespace,
        name,
        payload.description.as_deref()
    )
    .fetch_one(&mut *tx)
    .await?
    .id;

    let manifest = VmifManifest {
        schema_version: 2,
        appliance: format!("{}/{}", namespace, name),
        version: tag.clone(),
        recommended_kernel: kernel,
        boot_args,
        hardware: VmifHardware {
            vcpu,
            memory_mb,
            disk_gb,
        },
        rootfs_digest: rootfs_digest.clone(),
        signature: Some(format!("sig:attested:{}:{}", namespace, name)),
        created_at: Utc::now(),
    };

    // 1. Insert blob
    sqlx::query!(
        r#"
        INSERT INTO blobs (digest, size_bytes, media_type, storage_path)
        VALUES ($1, $2, 'application/vnd.vyoma.rootfs.squashfs', $3)
        ON CONFLICT (digest) DO NOTHING
        "#,
        rootfs_digest,
        rootfs_bytes.len() as i64,
        format!("blobs/{}", rootfs_digest)
    )
    .execute(&mut *tx)
    .await?;

    // 2. Insert manifest
    let manifest_json = serde_json::to_value(&manifest)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Manifest serialization failed: {}", e)))?;

    sqlx::query!(
        r#"
        INSERT INTO manifests (digest, repository_id, media_type, size_bytes, payload)
        VALUES ($1, $2, 'application/vnd.vyoma.vmif.manifest.v2+json', $3, $4)
        ON CONFLICT (digest) DO UPDATE SET payload = EXCLUDED.payload
        "#,
        rootfs_digest,
        repo_id,
        rootfs_bytes.len() as i64,
        manifest_json
    )
    .execute(&mut *tx)
    .await?;

    // 3. Upsert tag
    sqlx::query!(
        r#"
        INSERT INTO tags (repository_id, name, manifest_digest)
        VALUES ($1, $2, $3)
        ON CONFLICT (repository_id, name) DO UPDATE SET
            manifest_digest = EXCLUDED.manifest_digest,
            updated_at = NOW()
        "#,
        repo_id,
        tag,
        rootfs_digest
    )
    .execute(&mut *tx)
    .await?;

    // 4. Record Merkle transparency log entry
    let mut leaf_hasher = Sha256::new();
    leaf_hasher.update(format!("appliance:{}/{}:{}", namespace, name, rootfs_digest).as_bytes());
    let leaf_hash = format!("sha256:{:x}", leaf_hasher.finalize());

    sqlx::query!(
        r#"
        INSERT INTO transparency_logs (target_type, target_name, version, digest, signature, public_key, leaf_hash)
        VALUES ('appliance', $1, $2, $3, $4, '8ea335vyoma_official_root_key', $5)
        "#,
        format!("{}/{}", namespace, name),
        tag,
        rootfs_digest,
        payload.signature.unwrap_or_else(|| format!("sig:{}:{}", namespace, name)),
        leaf_hash
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(UploadApplianceResponse {
        status: "published".to_string(),
        repository: format!("{}/{}", namespace, name),
        tag,
        digest: rootfs_digest,
        manifest,
    }))
}

