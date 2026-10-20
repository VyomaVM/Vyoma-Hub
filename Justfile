set dotenv-load := true

# Default: List all available recipes
default:
    @just --list

# -------------------------------------------------------------
# 🐳 Infrastructure (Docker Compose)
# -------------------------------------------------------------

# Start local infrastructure (Postgres 16, Redis 7, MinIO S3)
up:
    docker compose -f docker/docker-compose.yml up -d
    @echo "✅ Postgres (5432), Redis (6379), MinIO (9000/9001) are running"

# Stop local infrastructure
down:
    docker compose -f docker/docker-compose.yml down

# Tail logs of local infrastructure services
logs service="":
    docker compose -f docker/docker-compose.yml logs -f {{service}}

# Reset all local database and storage volumes
clean: down
    docker volume prune -f
    rm -rf target/ apps/web/.next apps/web/node_modules

# -------------------------------------------------------------
# 🗄️ Database & Migrations
# -------------------------------------------------------------

# Run pending database migrations
migrate:
    cd apps/server && cargo sqlx migrate run

# Revert the latest database migration
migrate-revert:
    cd apps/server && cargo sqlx migrate revert

# Seed database with sample verified publishers, appliances, and kernels
seed:
    cd apps/server && cargo run --bin vyoma-hub-seed

# -------------------------------------------------------------
# 🚀 Development & Live Reload
# -------------------------------------------------------------

# Start the full local stack (Infra + Backend + Frontend)
dev: up
    npx -y concurrently -k -n "server,web" -c "magenta,cyan" \
      "just dev-server" \
      "just dev-web"

# Run only backend with hot reload (cargo watch)
dev-server: up
    cd apps/server && cargo watch -x 'run --bin vyoma-hub-server'

# Run only frontend (Next.js)
dev-web:
    pnpm --filter web dev

# -------------------------------------------------------------
# 🧪 Testing, Linting & Quality Gates
# -------------------------------------------------------------

# Run full quality check across Rust & TypeScript
check:
    cargo check --all-targets
    cargo clippy --all-targets -- -D warnings
    pnpm --filter web check
    pnpm --filter web lint

# Run all test suites across server and web
test:
    cargo test --all
    pnpm --filter web test

# Format all Rust and TypeScript code
fmt:
    cargo fmt
    pnpm --filter web format

# -------------------------------------------------------------
# 📦 Production Build
# -------------------------------------------------------------

# Build production artifacts for release
build:
    cargo build --release --bin vyoma-hub-server
    pnpm --filter web build
