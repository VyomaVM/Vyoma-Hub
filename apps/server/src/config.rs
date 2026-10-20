use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_force_path_style: bool,
    pub jwt_secret: String,
    pub token_issuer: String,
    pub token_expiry_hours: i64,
    pub hub_public_url: String,
    pub web_public_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:vyomapassword@localhost:5432/vyoma_hub".to_string());
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let s3_endpoint = env::var("S3_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:9000".to_string());
        let s3_region = env::var("S3_REGION")
            .unwrap_or_else(|_| "us-east-1".to_string());
        let s3_bucket = env::var("S3_BUCKET")
            .unwrap_or_else(|_| "vyoma-hub-blobs".to_string());
        let s3_access_key = env::var("S3_ACCESS_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string());
        let s3_secret_key = env::var("S3_SECRET_KEY")
            .unwrap_or_else(|_| "minioadminpassword".to_string());
        let s3_force_path_style = env::var("S3_FORCE_PATH_STYLE")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(true);
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "vyoma_hub_dev_secret_key_change_me_in_production_min_32_bytes_long!".to_string());
        let token_issuer = env::var("TOKEN_ISSUER")
            .unwrap_or_else(|_| "hub.vyoma.dev".to_string());
        let token_expiry_hours = env::var("TOKEN_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<i64>()?;
        let hub_public_url = env::var("HUB_PUBLIC_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        let web_public_url = env::var("WEB_PUBLIC_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        Ok(Self {
            host,
            port,
            database_url,
            redis_url,
            s3_endpoint,
            s3_region,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
            s3_force_path_style,
            jwt_secret,
            token_issuer,
            token_expiry_hours,
            hub_public_url,
            web_public_url,
        })
    }
}
