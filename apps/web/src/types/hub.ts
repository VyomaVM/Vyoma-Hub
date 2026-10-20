export interface CatalogItem {
  id: string;
  namespace: string;
  name: string;
  description: string | null;
  pull_count: number;
  star_count: number;
  is_verified: boolean;
  tags: string[];
  recommended_kernel: string | null;
  created_at: string;
  category?: "database" | "network" | "ai" | "security" | "runtime" | "os";
  vcpu?: number;
  memory_mb?: number;
  disk_gb?: number;
  readme?: string;
  pcr_measurement?: string;
  cve_clean?: boolean;
}

export interface KernelProfile {
  id: string;
  name: string;
  version: string;
  workload_type: string;
  vmlinuz_digest: string;
  config_digest?: string | null;
  ed25519_signature: string;
  ed25519_public_key: string;
  is_official: boolean;
  description: string;
  recommended_cmdline: string;
  created_at: string;
  boot_time_ms?: number;
  ram_footprint_mb?: number;
  optimizations?: string[];
  supported_cpus?: string[];
}

export interface UserToken {
  id: string;
  name: string;
  token?: string;
  token_preview: string;
  scopes: string[];
  created_at: string;
  last_used_at?: string | null;
}

export interface TransparencyLogItem {
  id: string;
  sequence_number: number;
  target_type: string;
  target_name: string;
  version: string;
  digest: string;
  signature: string;
  public_key: string;
  leaf_hash: string;
  created_at: string;
}

export interface TransparencyLogData {
  merkle_root: string;
  total_entries: number;
  entries: TransparencyLogItem[];
}

export interface Organization {
  id: string;
  name: string;
  display_name: string;
  description: string | null;
  is_verified: boolean;
  avatar_url: string | null;
  role?: string | null;
  created_at: string;
}

export interface OrgMember {
  user_id: string;
  username: string;
  full_name: string | null;
  role: "owner" | "admin" | "member" | "viewer" | string;
  joined_at: string;
}

export interface Webhook {
  id: string;
  org_id: string;
  url: string;
  events: string[];
  is_active: boolean;
  created_at: string;
}

export interface WebhookDelivery {
  id: string;
  webhook_id: string;
  event_type: string;
  status_code: number | null;
  request_payload: Record<string, unknown>;
  response_body: string | null;
  delivered_at: string;
}
