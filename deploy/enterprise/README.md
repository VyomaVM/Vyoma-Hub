# Vyoma Hub — Enterprise Air-Gapped Deployment Runbook

This directory contains production-ready deployment assets for operating **Vyoma Hub** in strict air-gapped, on-premise, or enterprise cloud VPC environments without outbound internet access.

---

## 1. Quick Start: Single-Node Enterprise (Docker Compose)

For edge appliances, on-premises datacenters, or development clusters:

```bash
cd deploy
docker compose -f docker-compose.enterprise.yml up -d
```

### Services Initialized
| Container Name | Service | Internal Port | External Port | Function |
|---|---|---|---|---|
| `vyoma-enterprise-gateway` | Nginx 1.25 | 80, 443 | 80, 443 | SSL Termination & Path Routing |
| `vyoma-enterprise-server` | Rust Core Engine | 8080 | - | OCI v2, VMIF Engine & Notary |
| `vyoma-enterprise-web` | Next.js 16 Web UI | 3000 | - | Catalog & Team Management UI |
| `vyoma-enterprise-postgres` | PostgreSQL 16 | 5432 | - | Persistent Metadata & Transparency Log |
| `vyoma-enterprise-redis` | Redis 7 AOF | 6379 | - | Job Queues & Token Revocation |
| `vyoma-enterprise-minio` | MinIO S3 | 9000, 9001 | - | Content-Addressed Rootfs Blob Store |

### Verification
```bash
# Verify health of all services
curl http://localhost/health
```

---

## 2. Kubernetes Enterprise Deployment (Helm Chart)

For Kubernetes (EKS, GKE, OpenShift, Rancher, or air-gapped K3s/RKE2):

```bash
# 1. Custom values file for your corporate domain
cat <<EOF > my-values.yaml
global:
  domain: hub.corp.internal

server:
  replicaCount: 3

ingress:
  className: nginx
  hosts:
    - host: hub.corp.internal
      paths:
        - path: /
          pathType: Prefix
EOF

# 2. Deploy via Helm
helm upgrade --install vyoma-hub ./deploy/helm/vyoma-hub -f my-values.yaml -n vyoma --create-namespace
```

---

## 3. Air-Gapped Image Import & Bundle Transfer

In true air-gapped environments, export images on a connected machine:

```bash
# Save images to tarball
docker save -o vyoma-hub-airgap-v0.1.0.tar \
  vyoma/hub-server:latest \
  vyoma/hub-web:latest \
  postgres:16-alpine \
  redis:7-alpine \
  cgr.dev/chainguard/minio:latest \
  nginx:alpine

# Transfer to bastion / secure jump host, then load:
docker load -i vyoma-hub-airgap-v0.1.0.tar
```

---

## 4. Disaster Recovery & Backup

### PostgreSQL Backup
```bash
docker exec vyoma-enterprise-postgres pg_dump -U postgres vyoma_hub > vyoma_hub_backup_$(date +%Y%m%d).sql
```

### MinIO S3 Blob Store Backup
```bash
# Sync all SHA-256 content-addressed blobs to cold storage
docker exec vyoma-enterprise-minio mc mirror /data/vyoma-hub-blobs /cold-storage/blobs/
```
