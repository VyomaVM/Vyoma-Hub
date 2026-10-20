use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User UUID
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciAccess {
    pub r#type: String, // e.g. "repository"
    pub name: String,   // e.g. "vyoma/postgres"
    pub actions: Vec<String>, // ["pull", "push"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciClaims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub access: Vec<OciAccess>,
}

pub fn mint_user_token(
    user_id: Uuid,
    username: &str,
    is_admin: bool,
    secret: &str,
    issuer: &str,
    expiry_hours: i64,
) -> Result<String, anyhow::Error> {
    let now = Utc::now();
    let exp = now + Duration::hours(expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        is_admin,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
        iss: issuer.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_user_token(token: &str, secret: &str) -> Result<Claims, anyhow::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn mint_oci_token(
    subject: &str,
    access: Vec<OciAccess>,
    secret: &str,
    issuer: &str,
    service: &str,
    expiry_minutes: i64,
) -> Result<String, anyhow::Error> {
    let now = Utc::now();
    let exp = now + Duration::minutes(expiry_minutes);

    let claims = OciClaims {
        sub: subject.to_string(),
        iss: issuer.to_string(),
        aud: service.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
        access,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_oci_token(token: &str, secret: &str) -> Result<OciClaims, anyhow::Error> {
    let mut validation = Validation::default();
    validation.validate_aud = false; // Aud may be service name
    let token_data = decode::<OciClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}
