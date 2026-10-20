pub mod extractor;
pub mod jwt;
pub mod password;
pub mod token;

pub use extractor::{AuthUser, MaybeAuthUser};
pub use jwt::{mint_oci_token, mint_user_token, verify_oci_token, verify_user_token, Claims, OciAccess, OciClaims};
pub use password::{hash_password, verify_password};
pub use token::{generate_personal_access_token, hash_token, GeneratedToken};

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "SuperSecretPassword123!";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash));
        assert!(!verify_password("WrongPassword!", &hash));
    }

    #[test]
    fn test_user_jwt_mint_and_verify() {
        let user_id = Uuid::new_v4();
        let username = "vyoma-admin";
        let secret = "super-secret-test-jwt-key-minimum-32-bytes!!";
        let issuer = "hub.vyoma.dev";

        let token = mint_user_token(user_id, username, true, secret, issuer, 2).unwrap();
        let claims = verify_user_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
        assert!(claims.is_admin);
    }

    #[test]
    fn test_pat_generation_and_hashing() {
        let pat = generate_personal_access_token();
        assert!(pat.raw_token.starts_with("vyh_"));
        assert_eq!(pat.prefix, pat.raw_token[..8]);
        assert_eq!(hash_token(&pat.raw_token), pat.token_hash);
    }

    #[test]
    fn test_oci_token_mint_and_verify() {
        let subject = "vyoma-user";
        let secret = "super-secret-test-jwt-key-minimum-32-bytes!!";
        let issuer = "hub.vyoma.dev";
        let service = "hub.vyoma.dev";

        let access = vec![OciAccess {
            r#type: "repository".to_string(),
            name: "vyoma/postgres".to_string(),
            actions: vec!["pull".to_string(), "push".to_string()],
        }];

        let token = mint_oci_token(subject, access, secret, issuer, service, 60).unwrap();
        let claims = verify_oci_token(&token, secret).unwrap();

        assert_eq!(claims.sub, subject);
        assert_eq!(claims.access.len(), 1);
        assert_eq!(claims.access[0].name, "vyoma/postgres");
        assert_eq!(claims.access[0].actions, vec!["pull", "push"]);
    }
}
