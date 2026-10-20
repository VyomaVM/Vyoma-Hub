-- ============================================================================
-- Migration: OCI-to-VMIF Conversion Jobs & Commercial Marketplace Licenses
-- ============================================================================

-- 1. Conversion Jobs Table
CREATE TABLE IF NOT EXISTS conversion_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_image VARCHAR(255) NOT NULL,
    target_appliance VARCHAR(255) NOT NULL,
    status VARCHAR(64) NOT NULL DEFAULT 'queued', -- 'queued', 'pulling', 'converting', 'completed', 'failed'
    assigned_kernel VARCHAR(64) NOT NULL DEFAULT 'universal-general',
    rootfs_digest VARCHAR(128),
    size_bytes BIGINT NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_conversion_status ON conversion_jobs(status);

-- 2. Commercial Licenses Table (Phase 4 Marketplace)
CREATE TABLE IF NOT EXISTS commercial_licenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    license_key VARCHAR(128) UNIQUE NOT NULL,
    org_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    appliance_name VARCHAR(128) NOT NULL,
    tier VARCHAR(64) NOT NULL DEFAULT 'enterprise', -- 'starter', 'pro', 'enterprise'
    max_nodes INT NOT NULL DEFAULT 10,
    max_vcpu_per_vm INT NOT NULL DEFAULT 64,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at TIMESTAMPTZ NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    signature VARCHAR(255) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_licenses_key ON commercial_licenses(license_key);
CREATE INDEX IF NOT EXISTS idx_licenses_org ON commercial_licenses(org_id);
