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
