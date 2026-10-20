use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    println!("🌱 Seeding Vyoma Hub with Certified Kernels & Official Appliances...");
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:vyomapassword@localhost:5432/vyoma_hub".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // 1. Generate Official Notary Signing Keypair (Ed25519)
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let notary_pubkey = hex::encode(verifying_key.as_bytes());
    println!("🔑 Official Notary Ed25519 Public Key: {}", notary_pubkey);

    // 2. Seed Official Organization
    sqlx::query!(
        r#"
        INSERT INTO organizations (name, display_name, description, is_verified)
        VALUES ('vyoma', 'Vyoma Official', 'Official hardware-isolated micro-OS appliances and certified kernels built by the Vyoma Core Team', TRUE)
        ON CONFLICT (name) DO UPDATE SET
            display_name = EXCLUDED.display_name,
            is_verified = TRUE
        "#
    )
    .execute(&pool)
    .await?;

    // 3. Seed Kernel Profiles
    let kernel_specs = [
        (
            "database",
            "6.6.30-vyoma",
            "Database",
            "sha256:7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b",
            "sha256:11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff",
            "Tuned for high IOPS databases (PostgreSQL, MongoDB). Features Transparent HugePages (2MB), Deadline I/O elevator, io_uring asynchronous backend, and zero swap overhead.",
            "console=ttyS0 root=/dev/vda rw elevator=deadline transparent_hugepage=always nohz_full=1-3 mitigations=off",
        ),
        (
            "edge-network",
            "6.6.30-vyoma",
            "EdgeNetwork",
            "sha256:8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c",
            "sha256:223344556677889900aabbccddeeff11223344556677889900aabbccddeeff22",
            "Engineered for sub-millisecond reverse proxies and edge gateways (Nginx, Envoy). Includes XDP, eBPF JIT compiler, TCP BBRv3 congestion control, and ultra-stripped memory footprint (12MB total boot overhead).",
            "console=ttyS0 root=/dev/vda rw tcp_congestion_control=bbr net.core.bpf_jit_enable=1 quiet",
        ),
        (
            "ai-inference",
            "6.6.30-vyoma",
            "AIInference",
            "sha256:9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d",
            "sha256:3344556677889900aabbccddeeff11223344556677889900aabbccddeeff33",
            "Optimized for GPU and NPU hardware acceleration (PyTorch, Ollama, TensorRT). Features VFIO PCI passthrough, host NUMA node pinning, PREEMPT_RT deterministic latency, and 1GB hugepages support.",
            "console=ttyS0 root=/dev/vda rw intel_iommu=on vfio_iommu_type1.allow_unsafe_interrupts=1 default_hugepagesz=1G hugepagesz=1G hugepages=8",
        ),
        (
            "hardened-security",
            "6.6.30-vyoma",
            "HardenedSecurity",
            "sha256:0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e",
            "sha256:44556677889900aabbccddeeff11223344556677889900aabbccddeeff44",
            "Military-grade security kernel for zero-trust workloads, key vaults, and multi-tenant hosting. Strict Lockdown integrity, SELinux enforcing, Kernel Memory Sanitizer, zero debugfs, and strict W^X memory policies.",
            "console=ttyS0 root=/dev/vda rw lockdown=integrity selinux=1 enforcing=1 slub_debug=FZP init_on_alloc=1 pti=on quiet",
        ),
        (
            "universal-general",
            "6.6.30-vyoma",
            "UniversalGeneral",
            "sha256:1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f",
            "sha256:556677889900aabbccddeeff11223344556677889900aabbccddeeff55",
            "Balanced general-purpose micro-kernel for generic web applications, Node.js, and background workers with dynamic ACPI CPU scaling and virtio-ballooning.",
            "console=ttyS0 root=/dev/vda rw quiet",
        ),
    ];

    for (name, version, workload, vmlinuz_digest, config_digest, description, cmdline) in kernel_specs {
        // Sign: name:version:vmlinuz_digest:config_digest
        let sign_payload = format!("{}:{}:{}:{}", name, version, vmlinuz_digest, config_digest);
        let signature = signing_key.sign(sign_payload.as_bytes());
        let sig_hex = hex::encode(signature.to_bytes());

        sqlx::query!(
            r#"
            INSERT INTO kernel_profiles (
                name, version, workload_type, vmlinuz_digest, config_digest,
                ed25519_signature, ed25519_public_key, is_official, description, recommended_cmdline
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, TRUE, $8, $9)
            ON CONFLICT (name, version) DO UPDATE SET
                workload_type = EXCLUDED.workload_type,
                vmlinuz_digest = EXCLUDED.vmlinuz_digest,
                config_digest = EXCLUDED.config_digest,
                ed25519_signature = EXCLUDED.ed25519_signature,
                ed25519_public_key = EXCLUDED.ed25519_public_key,
                description = EXCLUDED.description,
                recommended_cmdline = EXCLUDED.recommended_cmdline
            "#,
            name,
            version,
            workload,
            vmlinuz_digest,
            config_digest,
            sig_hex,
            notary_pubkey,
            description,
            cmdline
        )
        .execute(&pool)
        .await?;
        println!("  ✓ Seeded certified kernel: {} ({})", name, workload);
    }

    // 4. Seed Official Appliance Repositories
    let appliances = [
        (
            "postgres",
            "Official PostgreSQL 16 Hardware-Isolated Micro-OS Appliance",
            "# Vyoma PostgreSQL Appliance\n\nProduction-ready PostgreSQL 16 compiled into a read-only Squashfs rootfs backed by a dedicated Vyoma `database` kernel with direct I/O and HugePages.",
            14205,
            382,
            vec!["16-alpine", "latest", "16.3"],
        ),
        (
            "nginx",
            "High-Performance Edge Web Server & Reverse Proxy",
            "# Vyoma Nginx Edge Appliance\n\nUltra-low latency web server tuned with the Vyoma `edge-network` kernel, XDP zero-copy packet processing, and TCP BBRv3.",
            28910,
            612,
            vec!["1.26-alpine", "latest", "mainline"],
        ),
        (
            "redis",
            "In-Memory Key-Value Cache & Message Broker",
            "# Vyoma Redis Appliance\n\nSub-millisecond Redis 7.2 micro-OS appliance running with memory overcommit optimization and transparent hugepage allocation.",
            19540,
            490,
            vec!["7-alpine", "latest", "7.2.4"],
        ),
        (
            "ollama",
            "Hardware-Isolated Local LLM and Embedding Engine",
            "# Vyoma Ollama Appliance\n\nIsolated AI inference appliance tuned with the `ai-inference` kernel, direct GPU passthrough, and pinned NUMA execution.",
            8430,
            520,
            vec!["0.3.0", "latest"],
        ),
        (
            "node-runtime",
            "Hardened Node.js 22 LTS Micro-OS Runtime",
            "# Vyoma Node.js Runtime\n\nMinimal, attack-surface-reduced Node.js runtime for microservices and API gateways.",
            9120,
            245,
            vec!["22-alpine", "latest"],
        ),
    ];

    for (name, desc, readme, pulls, stars, tags) in appliances {
        let repo_id = sqlx::query!(
            r#"
            INSERT INTO repositories (namespace, name, description, readme, is_public, pull_count, star_count)
            VALUES ('vyoma', $1, $2, $3, TRUE, $4, $5)
            ON CONFLICT (namespace, name) DO UPDATE SET
                description = EXCLUDED.description,
                readme = EXCLUDED.readme,
                pull_count = EXCLUDED.pull_count,
                star_count = EXCLUDED.star_count
            RETURNING id
            "#,
            name,
            desc,
            readme,
            pulls as i64,
            stars as i32
        )
        .fetch_one(&pool)
        .await?
        .id;

        // Seed a sample manifest
        let manifest_digest = format!("sha256:{:0>64}", name.len() * 1111);
        let sample_manifest = json!({
            "schemaVersion": 2,
            "mediaType": "application/vnd.vyoma.vmif.manifest.v1+json",
            "config": {
                "mediaType": "application/vnd.vyoma.vmif.config.v1+json",
                "digest": format!("sha256:{:0>64}", name.len() * 2222),
                "size": 1420
            },
            "layers": [
                {
                    "mediaType": "application/vnd.vyoma.image.rootfs.squashfs",
                    "digest": format!("sha256:{:0>64}", name.len() * 3333),
                    "size": 48291040
                }
            ]
        });

        sqlx::query!(
            r#"
            INSERT INTO manifests (digest, repository_id, media_type, size_bytes, payload)
            VALUES ($1, $2, 'application/vnd.vyoma.vmif.manifest.v1+json', 1024, $3)
            ON CONFLICT (digest) DO NOTHING
            "#,
            manifest_digest,
            repo_id,
            sample_manifest
        )
        .execute(&pool)
        .await?;

        for tag in tags {
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
                manifest_digest
            )
            .execute(&pool)
            .await?;
        }

        println!("  ✓ Seeded official appliance: vyoma/{}", name);
    }

    println!("✨ Seed completed successfully! Vyoma Hub is fully provisioned.");
    Ok(())
}
