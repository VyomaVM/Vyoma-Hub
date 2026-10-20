# Vyoma Hub

**Vyoma Hub** (`hub.vyoma.dev`) is the official App Store, appliance registry, and workload-tuned kernel repository for the [Vyoma MicroVM Ecosystem](https://github.com/VyomaVM/Vyoma).

Unlike standard container registries that distribute flat filesystems on a shared host kernel, Vyoma Hub distributes **hardware-isolated micro-OS appliances** (VMIF) with **certified, workload-specific Linux kernels** (Database, Network/Edge, AI/ML, and Confidential Computing) and end-to-end **Ed25519 cryptographic signing**.

---

## 🏛️ System Architecture & Specifications

Complete technical blueprints and design documents:
- **[System Architecture & Blueprint](docs/system-architecture.md)**: Full-stack platform design, dual-engine registry (OCI v2 + VMIF), auth, notary service, and database schema.
- **[Kernel Catalog & VMIF Appliance Spec](docs/architecture/kernel-catalog-and-vmif.md)**: VMIF v2 layout, workload kernel matrix, and hardware-rooted attestation pipeline.

---

## 📦 Monorepo Structure

```
Vyoma-Hub/
├── apps/
│   ├── web/              # Frontend web application (Next.js / TypeScript / React)
│   │                     # Appliance Explorer, Kernel Catalog, Org Dashboard, Scorecards
│   └── server/           # Backend API & Registry Service
│                         # OCI v2 distribution, VMIF endpoints, Auth, Notary signer
├── packages/
│   ├── types/            # Shared TypeScript / JSON schema definitions
│   └── config/           # Shared linter and environment configs
├── docker/
│   └── docker-compose.yml # Local development stack (PostgreSQL 16, Redis, MinIO S3)
├── docs/                 # Architectural specifications and API documentation
└── README.md
```

---

## 🚀 Quick Links & Organizations

- **Engine Core:** [VyomaVM/Vyoma](https://github.com/VyomaVM/Vyoma)
- **Marketing & Docs:** [VyomaVM/Vyom-Site](https://github.com/VyomaVM/Vyom-Site)
- **Organization:** [VyomaVM on GitHub](https://github.com/VyomaVM)
- **License:** Apache 2.0
