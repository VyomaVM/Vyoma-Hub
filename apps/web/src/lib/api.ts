import { CatalogItem, KernelProfile } from "@/types/hub";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8081";

export const FALLBACK_KERNELS: KernelProfile[] = [
  {
    id: "k-db-001",
    name: "database",
    version: "6.6.30-vyoma",
    workload_type: "Database",
    vmlinuz_digest: "sha256:7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b",
    config_digest: "sha256:11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff",
    ed25519_signature: "e38a91bc7d2ef149ab01cd99876f543210aaabbbcccdddeeefff112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233445566",
    ed25519_public_key: "239b0c01e974044d748ff780c9389fb5b626bde95776c0cb4351c2e28460a6d7",
    is_official: true,
    description: "Stripped kernel optimized for high-IOPS relational and KV databases (PostgreSQL, MongoDB, ClickHouse). Features io_uring zero-copy and direct NVMe passthrough.",
    recommended_cmdline: "console=hvc0 root=/dev/vda rw quiet iomem=relaxed mitigations=off io_uring=1 elevator=none",
    created_at: "2026-10-20T00:00:00Z",
    boot_time_ms: 185,
    ram_footprint_mb: 26,
    optimizations: ["io_uring NVMe zero-copy", "Disabled pagecache readahead", "Preemptive RT locks", "Bcachefs / ext4 direct I/O"],
    supported_cpus: ["x86_64", "AMD SEV-SNP", "Intel TDX"],
  },
  {
    id: "k-edge-002",
    name: "edge-network",
    version: "6.6.30-vyoma",
    workload_type: "EdgeNetwork",
    vmlinuz_digest: "sha256:8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c",
    config_digest: "sha256:2233445566778899aabbccddeeff00112233445566778899aabbccddeeff0011",
    ed25519_signature: "f49b02cd8e3fa250bc12de00987a654321bbbcccddeeefff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff0011223344556677",
    ed25519_public_key: "239b0c01e974044d748ff780c9389fb5b626bde95776c0cb4351c2e28460a6d7",
    is_official: true,
    description: "Ultra-low latency networking kernel with stripped drivers and in-kernel eBPF / XDP wire-speed packet steering for NGINX, Envoy, and API gateways.",
    recommended_cmdline: "console=hvc0 root=/dev/vda rw quiet bpf_jit_enable=1 net.core.busy_read=50",
    created_at: "2026-10-20T00:00:00Z",
    boot_time_ms: 165,
    ram_footprint_mb: 22,
    optimizations: ["eBPF / XDP wire-speed bypass", "Stripped audio/USB/display", "BBR3 Congestion Control", "Multi-queue virtio-net RSS"],
    supported_cpus: ["x86_64", "aarch64", "AWS Graviton 3/4"],
  },
  {
    id: "k-ai-003",
    name: "ai-inference",
    version: "6.6.30-vyoma",
    workload_type: "AIInference",
    vmlinuz_digest: "sha256:9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d",
    config_digest: "sha256:33445566778899aabbccddeeff00112233445566778899aabbccddeeff001122",
    ed25519_signature: "a50c13de9f4ab361cd23ef11098b765432cccdddeeefff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff001122334455667788",
    ed25519_public_key: "239b0c01e974044d748ff780c9389fb5b626bde95776c0cb4351c2e28460a6d7",
    is_official: true,
    description: "Built for LLM token generation, PyTorch, Ollama, and tensor workloads. Pre-configured with HugeTLB 1GB memory pages and direct VFIO PCIe GPU passthrough.",
    recommended_cmdline: "console=hvc0 root=/dev/vda rw quiet default_hugepagesz=1G hugepagesz=1G hugepages=16 vfio-pci.ids=10de:2684",
    created_at: "2026-10-20T00:00:00Z",
    boot_time_ms: 210,
    ram_footprint_mb: 32,
    optimizations: ["HugeTLB 1GB memory pages", "VFIO PCIe GPU direct passthrough", "AVX-512 vector acceleration", "NUMA node binding"],
    supported_cpus: ["x86_64", "NVIDIA H100/A100", "AMD Instinct"],
  },
  {
    id: "k-sec-004",
    name: "hardened-security",
    version: "6.6.30-vyoma",
    workload_type: "HardenedSecurity",
    vmlinuz_digest: "sha256:0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e",
    config_digest: "sha256:445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233",
    ed25519_signature: "b61d24ef0a5bc472de34fa22109c876543dddeeefff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899",
    ed25519_public_key: "239b0c01e974044d748ff780c9389fb5b626bde95776c0cb4351c2e28460a6d7",
    is_official: true,
    description: "Zero-trust banking and secret-vault kernel. Complete kernel module loading disabled, strict seccomp filters, stack-canaries, and vTPM2 PCR measurement.",
    recommended_cmdline: "console=hvc0 root=/dev/vda rw quiet lockdown=integrity module.sig_enforce=1 slab_nomerge slub_debug=FZP pti=on",
    created_at: "2026-10-20T00:00:00Z",
    boot_time_ms: 195,
    ram_footprint_mb: 24,
    optimizations: ["Kernel lockdown=integrity", "Strict Seccomp & BPF filter", "vTPM2 PCR measurement", "Disabled dynamic module loading"],
    supported_cpus: ["x86_64", "Intel SGX / TDX", "Nitro Enclaves"],
  },
  {
    id: "k-uni-005",
    name: "universal-general",
    version: "6.6.30-vyoma",
    workload_type: "UniversalGeneral",
    vmlinuz_digest: "sha256:1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f",
    config_digest: "sha256:5566778899aabbccddeeff00112233445566778899aabbccddeeff0011223344",
    ed25519_signature: "c72e35fa1b6cd583ef45ab33210d987654eeefff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899aa",
    ed25519_public_key: "239b0c01e974044d748ff780c9389fb5b626bde95776c0cb4351c2e28460a6d7",
    is_official: true,
    description: "Standard production microVM kernel balancing ultra-fast cold boots (<220ms) with broad POSIX compatibility across web services and worker nodes.",
    recommended_cmdline: "console=hvc0 root=/dev/vda rw quiet",
    created_at: "2026-10-20T00:00:00Z",
    boot_time_ms: 220,
    ram_footprint_mb: 28,
    optimizations: ["Optimized virtio bus drivers", "Stripped legacy x86 PIC/IDE", "Fast memory ballooning", "Zstandard rootfs decompressor"],
    supported_cpus: ["x86_64", "aarch64"],
  },
];

export const FALLBACK_APPLIANCES: CatalogItem[] = [
  {
    id: "app-pg-001",
    namespace: "vyoma",
    name: "postgres",
    description: "Enterprise PostgreSQL 16 tuned for hardware-isolated microVMs with dedicated database kernel and pre-warmed shared memory.",
    pull_count: 48920,
    star_count: 1420,
    is_verified: true,
    tags: ["latest", "16-alpine", "16.4"],
    recommended_kernel: "database",
    created_at: "2026-10-20T00:00:00Z",
    category: "database",
    vcpu: 2,
    memory_mb: 512,
    disk_gb: 4,
    pcr_measurement: "f290d8a1c97e14f82638b91a70cde51203498afe",
    cve_clean: true,
    readme: `# PostgreSQL 16 MicroVM Appliance

The official Vyoma hardware-isolated PostgreSQL appliance. Runs directly on the Cloud Hypervisor KVM hypervisor with direct NVMe block I/O.

### Quick Start
\`\`\`bash
vyoma run hub.vyoma.dev/vyoma/postgres:latest --kernel-profile database --vcpu 2 --memory 512M -p 5432:5432
\`\`\`

### Highlights
- **Cold Boot:** 185ms to accept SQL connections
- **Memory Overhead:** Only 26MB host RAM
- **Direct I/O:** io_uring zero-copy bypasses guest pagecache for max throughput
- **Signed:** Ed25519 cryptographic attestation verified
`,
  },
  {
    id: "app-ollama-002",
    namespace: "vyoma",
    name: "ollama",
    description: "Local AI inference server running Llama 3, Mistral, and DeepSeek with GPU passthrough and HugeTLB 1GB pages.",
    pull_count: 36410,
    star_count: 2180,
    is_verified: true,
    tags: ["latest", "0.3.14", "gpu-cuda"],
    recommended_kernel: "ai-inference",
    created_at: "2026-10-20T00:00:00Z",
    category: "ai",
    vcpu: 4,
    memory_mb: 4096,
    disk_gb: 20,
    pcr_measurement: "b8472910fa37e2049c58b1049281734910298a44",
    cve_clean: true,
    readme: `# Ollama AI MicroVM Appliance

Deploy local LLMs in an isolated MicroVM with bare-metal GPU performance and zero virtualization penalty.

### Quick Start
\`\`\`bash
vyoma run hub.vyoma.dev/vyoma/ollama:latest --kernel-profile ai-inference --vcpu 4 --memory 4G -p 11434:11434
\`\`\`

### Features
- Direct VFIO PCIe NVIDIA GPU passthrough
- 1GB HugeTLB memory allocations for prompt cache
- Instant time-machine snapshot and branch support
`,
  },
  {
    id: "app-nginx-003",
    namespace: "vyoma",
    name: "nginx",
    description: "High-performance HTTP/3 edge reverse proxy and TLS terminator with eBPF/XDP network steering and stripped kernel.",
    pull_count: 92400,
    star_count: 980,
    is_verified: true,
    tags: ["latest", "1.27-alpine", "mainline"],
    recommended_kernel: "edge-network",
    created_at: "2026-10-20T00:00:00Z",
    category: "network",
    vcpu: 1,
    memory_mb: 256,
    disk_gb: 1,
    pcr_measurement: "7a892b104958ce4910298a48b910485729104829",
    cve_clean: true,
    readme: `# NGINX MicroVM Appliance

Ultra-lightweight edge router booting in 165ms.

### Quick Start
\`\`\`bash
vyoma run hub.vyoma.dev/vyoma/nginx:latest --kernel-profile edge-network --vcpu 1 --memory 256M -p 80:80 -p 443:443
\`\`\`
`,
  },
  {
    id: "app-redis-004",
    namespace: "vyoma",
    name: "redis",
    description: "In-memory cache and message broker operating in an isolated hardware boundary with sub-millisecond latency.",
    pull_count: 57120,
    star_count: 1150,
    is_verified: true,
    tags: ["latest", "7.4-alpine", "valkey-compat"],
    recommended_kernel: "database",
    created_at: "2026-10-20T00:00:00Z",
    category: "database",
    vcpu: 2,
    memory_mb: 512,
    disk_gb: 2,
    pcr_measurement: "4928104829104857291048593029485710294857",
    cve_clean: true,
    readme: `# Redis 7.4 MicroVM Appliance

High-throughput in-memory data store with dedicated kernel memory manager.

### Quick Start
\`\`\`bash
vyoma run hub.vyoma.dev/vyoma/redis:latest --kernel-profile database --vcpu 2 --memory 512M -p 6379:6379
\`\`\`
`,
  },
  {
    id: "app-node-005",
    namespace: "vyoma",
    name: "node-runtime",
    description: "Production Node.js 22 runtime for SSR, Next.js, and background API services with V8 optimization flags.",
    pull_count: 41200,
    star_count: 860,
    is_verified: true,
    tags: ["latest", "22-alpine", "20-lts"],
    recommended_kernel: "universal-general",
    created_at: "2026-10-20T00:00:00Z",
    category: "runtime",
    vcpu: 2,
    memory_mb: 1024,
    disk_gb: 5,
    pcr_measurement: "9029485710294857291048593029485710294857",
    cve_clean: true,
    readme: `# Node.js 22 MicroVM Appliance

Secure hardware-isolated JavaScript runtime.

### Quick Start
\`\`\`bash
vyoma run hub.vyoma.dev/vyoma/node-runtime:latest --vcpu 2 --memory 1G -p 3000:3000
\`\`\`
`,
  },
];

export async function getCatalog(query?: string, category?: string): Promise<{ items: CatalogItem[]; fromBackend: boolean }> {
  try {
    const params = new URLSearchParams();
    if (query) params.append("q", query);
    if (category && category !== "all") params.append("category", category);

    const res = await fetch(`${API_BASE}/api/v1/catalog?${params.toString()}`, {
      cache: "no-store",
    });

    if (res.ok) {
      const data = await res.json();
      if (Array.isArray(data) && data.length > 0) {
        return { items: data, fromBackend: true };
      }
    }
  } catch {
    // Backend offline or compiling, return enriched seed data
  }

  // Filter fallback data client-side
  let filtered = [...FALLBACK_APPLIANCES];
  if (query) {
    const q = query.toLowerCase();
    filtered = filtered.filter(
      (a) =>
        a.name.toLowerCase().includes(q) ||
        a.namespace.toLowerCase().includes(q) ||
        (a.description && a.description.toLowerCase().includes(q))
    );
  }
  if (category && category !== "all") {
    filtered = filtered.filter((a) => a.category === category);
  }

  return { items: filtered, fromBackend: false };
}

export async function getKernels(): Promise<{ kernels: KernelProfile[]; fromBackend: boolean }> {
  try {
    const res = await fetch(`${API_BASE}/api/v1/kernels`, {
      cache: "no-store",
    });

    if (res.ok) {
      const data = await res.json();
      if (Array.isArray(data) && data.length > 0) {
        return { kernels: data, fromBackend: true };
      }
    }
  } catch {
    // Return fallback
  }

  return { kernels: FALLBACK_KERNELS, fromBackend: false };
}

export async function checkBackendHealth(): Promise<boolean> {
  try {
    const res = await fetch(`${API_BASE}/health`, { cache: "no-store" });
    return res.ok;
  } catch {
    return false;
  }
}
