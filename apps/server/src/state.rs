use crate::config::Config;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::config::{Builder, Region};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: sqlx::PgPool,
    pub redis: redis::aio::ConnectionManager,
    pub s3: aws_sdk_s3::Client,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, anyhow::Error> {
        let config = Arc::new(config);

        // 1. PostgreSQL Connection Pool
        let db = PgPoolOptions::new()
            .max_connections(50)
            .connect(&config.database_url)
            .await?;
        tracing::info!("Connected to PostgreSQL at {}", config.database_url);

        // 2. Redis Connection Manager
        let redis_client = redis::Client::open(config.redis_url.as_str())?;
        let redis = redis::aio::ConnectionManager::new(redis_client).await?;
        tracing::info!("Connected to Redis at {}", config.redis_url);

        // 3. AWS S3 / MinIO Client
        let creds = Credentials::new(
            &config.s3_access_key,
            &config.s3_secret_key,
            None,
            None,
            "static",
        );
        let s3_conf = Builder::new()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(config.s3_region.clone()))
            .endpoint_url(&config.s3_endpoint)
            .force_path_style(config.s3_force_path_style)
            .credentials_provider(creds)
            .build();
        let s3 = aws_sdk_s3::Client::from_conf(s3_conf);
        tracing::info!(
            "Configured S3 Client pointing to endpoint {} (bucket: {})",
            config.s3_endpoint,
            config.s3_bucket
        );

        let state = Self {
            config,
            db,
            redis,
            s3,
        };

        // Ensure default storage bucket exists
        state.ensure_bucket().await.ok();

        Ok(state)
    }

    pub async fn ensure_bucket(&self) -> Result<(), anyhow::Error> {
        let bucket = &self.config.s3_bucket;
        match self.s3.head_bucket().bucket(bucket).send().await {
            Ok(_) => {
                tracing::info!("S3 bucket '{}' verified", bucket);
            }
            Err(_) => {
                tracing::info!("S3 bucket '{}' not found, creating...", bucket);
                self.s3.create_bucket().bucket(bucket).send().await?;
                tracing::info!("S3 bucket '{}' created successfully", bucket);
            }
        }
        Ok(())
    }
}
