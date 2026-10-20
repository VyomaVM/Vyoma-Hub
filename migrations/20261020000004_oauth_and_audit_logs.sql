-- ============================================================================
-- Migration: OAuth Accounts & Enterprise Audit Logs
-- ============================================================================

-- 1. Relax password_hash NOT NULL to allow OAuth users without passwords
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- 2. OAuth Accounts
CREATE TABLE IF NOT EXISTS user_oauth_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(32) NOT NULL, -- 'github', 'gitlab', 'oidc'
    provider_user_id VARCHAR(255) NOT NULL,
    profile_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (provider, provider_user_id)
);

CREATE INDEX IF NOT EXISTS idx_oauth_user ON user_oauth_accounts(user_id);

-- 3. Enterprise Security Audit Logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_username VARCHAR(64) NOT NULL,
    action VARCHAR(128) NOT NULL, -- 'org.member_add', 'org.member_remove', 'image.push', 'webhook.test', 'token.create'
    target_type VARCHAR(64) NOT NULL, -- 'repository', 'member', 'webhook', 'token'
    target_id VARCHAR(255),
    details JSONB,
    ip_address VARCHAR(45),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_org ON audit_logs(org_id);
CREATE INDEX IF NOT EXISTS idx_audit_created ON audit_logs(created_at DESC);
