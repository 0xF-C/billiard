use bcrypt::{hash, verify, DEFAULT_COST};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Hash password with bcrypt
pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_else(|_| "".to_string())
}

/// Verify password against bcrypt hash
pub fn verify_password(password: &str, hash_str: &str) -> bool {
    verify(password, hash_str).unwrap_or(false)
}

/// Generate a random UUID token
pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Sign a token with HMAC-SHA256 to prevent tampering
/// Returns "token.signature"
pub fn hmac_sign(token: &str) -> String {
    let secret = std::env::var("BILLIARD_SECRET").unwrap_or_else(|_| "billiard-manager-default-secret-2024".to_string());
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key");
    mac.update(token.as_bytes());
    let result = mac.finalize();
    let sig = hex::encode(result.into_bytes());
    format!("{}.{}", token, sig)
}

/// Verify an HMAC-signed token
/// Returns the original token if valid, None otherwise
#[allow(dead_code)]
pub fn hmac_verify(signed_token: &str) -> Option<String> {
    let parts: Vec<&str> = signed_token.splitn(2, '.').collect();
    if parts.len() != 2 {
        return None;
    }
    let token = parts[0];
    let sig = parts[1];
    
    let secret = std::env::var("BILLIARD_SECRET").unwrap_or_else(|_| "billiard-manager-default-secret-2024".to_string());
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key");
    mac.update(token.as_bytes());
    let result = mac.finalize();
    let expected = hex::encode(result.into_bytes());
    
    if sig == expected {
        Some(token.to_string())
    } else {
        None
    }
}
