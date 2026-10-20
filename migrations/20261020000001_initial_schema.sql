-- ============================================================================
-- Vyoma Hub Initial Production Schema
-- ============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(64) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(128),
    avatar_url TEXT,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Organizations
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) UNIQUE NOT NULL,
    display_name VARCHAR(128) NOT NULL,
    description TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_orgs_name ON organizations(name);

-- Organization Memberships
CREATE TABLE IF NOT EXISTS org_members (
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(32) NOT NULL DEFAULT 'member', -- 'owner', 'admin', 'member'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (org_id, user_id)
);

-- Repositories (Appliance & Image Catalogs)
CREATE TABLE IF NOT EXISTS repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    namespace VARCHAR(64) NOT NULL, -- user or organization name
    name VARCHAR(128) NOT NULL,
    description TEXT,
    readme TEXT,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    pull_count BIGINT NOT NULL DEFAULT 0,
    star_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (namespace, name)
);

CREATE INDEX IF NOT EXISTS idx_repos_namespace_name ON repositories(namespace, name);
CREATE INDEX IF NOT EXISTS idx_repos_public ON repositories(is_public);

-- Content-Addressed Blobs (S3 Storage References)
CREATE TABLE IF NOT EXISTS blobs (
    digest VARCHAR(128) PRIMARY KEY, -- e.g. sha256:...
    size_bytes BIGINT NOT NULL,
    media_type VARCHAR(128) NOT NULL,
    storage_path TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- OCI & VMIF Manifests
CREATE TABLE IF NOT EXISTS manifests (
    digest VARCHAR(128) PRIMARY KEY, -- sha256:...
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    media_type VARCHAR(128) NOT NULL,
    size_bytes BIGINT NOT NULL,
    config_digest VARCHAR(128),
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_manifests_repo ON manifests(repository_id);

-- Manifest to Blob Associations (Layers)
CREATE TABLE IF NOT EXISTS manifest_blobs (
    manifest_digest VARCHAR(128) NOT NULL REFERENCES manifests(digest) ON DELETE CASCADE,
    blob_digest VARCHAR(128) NOT NULL REFERENCES blobs(digest) ON DELETE CASCADE,
    ordinal INT NOT NULL DEFAULT 0,
    PRIMARY KEY (manifest_digest, blob_digest)
);

-- Repository Tags
CREATE TABLE IF NOT EXISTS tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    manifest_digest VARCHAR(128) NOT NULL REFERENCES manifests(digest) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (repository_id, name)
);

CREATE INDEX IF NOT EXISTS idx_tags_repo ON tags(repository_id);
CREATE INDEX IF NOT EXISTS idx_tags_manifest ON tags(manifest_digest);

-- Personal Access Tokens (CLI & API Auth)
CREATE TABLE IF NOT EXISTS api_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    token_hash VARCHAR(255) NOT NULL,
    token_prefix VARCHAR(16) NOT NULL, -- e.g. "vyh_9a8b"
    scopes TEXT[] NOT NULL DEFAULT '{"read", "write"}',
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_api_tokens_user ON api_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_api_tokens_prefix ON api_tokens(token_prefix);

-- Kernel Profiles Catalog (Official & Community Workload-Tuned Kernels)
CREATE TABLE IF NOT EXISTS kernel_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL, -- e.g. 'database', 'edge-network', 'ai-inference'
    version VARCHAR(32) NOT NULL, -- e.g. '6.6.30-vyoma'
    workload_type VARCHAR(64) NOT NULL, -- 'Database', 'EdgeNetwork', 'AIInference', 'HardenedSecurity', 'UniversalGeneral'
    vmlinuz_digest VARCHAR(128) NOT NULL,
    config_digest VARCHAR(128),
    ed25519_signature TEXT NOT NULL,
    ed25519_public_key TEXT NOT NULL,
    is_official BOOLEAN NOT NULL DEFAULT TRUE,
    description TEXT NOT NULL,
    recommended_cmdline TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (name, version)
);

CREATE INDEX IF NOT EXISTS idx_kernel_workload ON kernel_profiles(workload_type);
CREATE INDEX IF NOT EXISTS idx_kernel_name_version ON kernel_profiles(name, version);
