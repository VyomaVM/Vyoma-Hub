# Vyoma Hub

**Vyoma Hub** (`hub.vyoma.dev`) is the official App Store, appliance registry, and workload-tuned kernel repository for the [Vyoma MicroVM Ecosystem](https://github.com/VyomaVM/Vyoma).

Unlike standard container registries that distribute flat filesystems on a shared host kernel, Vyoma Hub distributes **hardware-isolated micro-OS appliances** (VMIF) with **certified, workload-specific Linux kernels** (Database, Network/Edge, AI/ML, and Confidential Computing) and end-to-end **Ed25519 cryptographic signing**.

---

## 🏛️ System Architecture & Specifications

Complete technical blueprints and design documents:
- **[System Architecture & Blueprint](docs/system-architecture.md)**: Full-stack platform design, dual-engine registry (OCI v2 + VMIF), auth, notary service, and database schema.
- **[Kernel Catalog & VMIF Appliance Spec](docs/architecture/kernel-catalog-and-vmif.md)**: VMIF v2 layout, workload kernel matrix, and hardware-rooted attestation pipeline.

---

## 🛠️ Ratified Technology Stack

| Layer | Technology | Key Dependencies & Rationale |
|---|---|---|
| **Backend API & Registry** | **Rust 1.80+** | `axum` 0.7, `tokio`, `sqlx`, `tower-http`. Direct code sharing with `crates/vyoma-image` (reusing `VmifManifest`, `KernelCatalog`, `SigningKeyPair`, `TrustPolicy`). |
| **Frontend Web App** | **Next.js 15 (React 19)** | TypeScript, App Router, SSR for appliance SEO, Vanilla CSS / CSS Modules. |
| **Master Task Runner** | **`just` (`Justfile`)** | Modern Rust/systems command runner unifying Docker, Cargo, and pnpm (`just dev`, `just check`). |
| **Frontend Package Manager** | **`pnpm` Workspaces (v9+)** | Strict dependency isolation for `apps/web` and `packages/shared`. Replaces Turborepo bloat. |
| **Primary Database** | **PostgreSQL 16** | Relational ACID guarantees for orgs/users/repos, JSONB storage for manifests. |
| **Cache & Queues** | **Redis 7** | Token rate limiting, session cache, async task queuing. |
| **Blob Storage** | **S3-Compatible** | Cloudflare R2 in production (zero egress fees), MinIO locally. |

---

## 📦 Monorepo Structure

```
Vyoma-Hub/
├── apps/
│   ├── web/              # Frontend web application (Next.js 15 / TypeScript / React 19)
│   │                     # Appliance Explorer, Kernel Catalog, Org Dashboard, Scorecards
│   └── server/           # Backend API & Registry Service (Rust / Axum)
│                         # OCI v2 distribution, VMIF endpoints, Auth, Notary signer
├── packages/
│   ├── types/            # Shared TypeScript / JSON schema definitions
│   └── config/           # Shared linter and environment configs
├── docker/
│   └── docker-compose.yml # Local development stack (PostgreSQL 16, Redis, MinIO S3)
├── docs/                 # Architectural specifications and API documentation
├── pnpm-workspace.yaml   # pnpm workspace definition
├── package.json          # Monorepo root scripts
├── Justfile              # Master command runner recipes (just dev, just check)
└── README.md
```

---

## 🌿 Git Strategy & Engineering Workflow

Vyoma Hub strictly adheres to the Git strategy established across the Vyoma ecosystem:

- **Primary Branches**:
  - `main`: Production release branch. Protected. Zero direct commits. Deploys to `hub.vyoma.dev`.
  - `develop`: Primary integration branch. All feature branches merge here first.
- **Working Branches**:
  - `feat/<scope>-<description>` (e.g. `feat/registry-oci-v2-handlers`)
  - `fix/<scope>-<description>` (e.g. `fix/auth-token-expiration`)
  - `docs/<description>` (e.g. `docs/api-token-specification`)
  - `chore/<description>` (e.g. `chore/upgrade-pnpm-dependencies`)
- **Commit Standards**:
  - Strict Conventional Commits: `<type>(<scope>): <summary>` (e.g. `feat(server): implement OCI v2 manifest negotiation`).
- **CI/CD Quality Gates**:
  - Backend: `cargo check`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt -- --check`.
  - Frontend: `pnpm --filter web check`, `pnpm --filter web lint`, `pnpm --filter web build`.

---

## 🚀 Quick Links & Organizations

- **Engine Core:** [VyomaVM/Vyoma](https://github.com/VyomaVM/Vyoma)
- **Marketing & Docs:** [VyomaVM/Vyom-Site](https://github.com/VyomaVM/Vyom-Site)
- **Organization:** [VyomaVM on GitHub](https://github.com/VyomaVM)
- **License:** Apache 2.0
