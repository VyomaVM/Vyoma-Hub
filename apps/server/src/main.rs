pub mod auth;
pub mod catalog;
pub mod config;
pub mod error;
pub mod registry;
pub mod routes;
pub mod state;

use config::Config;
use state::AppState;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,vyoma_hub_server=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Vyoma Hub Engine...");
    let config = Config::from_env()?;
    let state = AppState::new(config.clone()).await?;

    let app = routes::build_router(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("🚀 Vyoma Hub Server online & listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
