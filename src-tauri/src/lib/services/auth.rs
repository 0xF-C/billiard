use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::{hash_password, verify_password, generate_token, hmac_sign};
use rusqlite::params;

pub fn login(username: &str, password: &str) -> LoginResponse {
    let conn = DB.lock();

    // Try to get bcrypt_hash first (new), fall back to old password_hash+salt
    let result: Result<(i64, Option<String>, Option<String>, Option<String>, String, Option<i32>), _> = conn.query_row(
        "SELECT id, bcrypt_hash, password_hash, salt, role, first_login FROM users WHERE username = ?1",
        params![username],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
    );

    match result {
        Ok((user_id, bcrypt_hash, old_hash, old_salt, role, first_login)) => {
            let password_valid = if let Some(ref bh) = bcrypt_hash {
                // Use bcrypt
                verify_password(password, bh)
            } else if let (Some(ref oh), Some(ref os)) = (old_hash, old_salt) {
                // Legacy SHA-256, verify and upgrade to bcrypt
                let legacy_hash = hash_password_legacy(password, os);
                let valid = &legacy_hash == oh;
                if valid {
                    // Upgrade to bcrypt on successful legacy login
                    let new_hash = hash_password(password);
                    conn.execute(
                        "UPDATE users SET bcrypt_hash = ?1, first_login = 1 WHERE id = ?2",
                        params![new_hash, user_id],
                    ).ok();
                }
                valid
            } else {
                false
            };

            if password_valid {
                let token = generate_token();
                let signed_token = hmac_sign(&token);
                let expires = chrono::Utc::now() + chrono::Duration::days(7);
                
                conn.execute(
                    "INSERT INTO user_tokens (user_id, token, expires_at) VALUES (?1, ?2, ?3)",
                    params![user_id, signed_token, expires.to_rfc3339()],
                ).ok();

                LoginResponse {
                    success: true,
                    token: Some(signed_token),
                    user: Some(User {
                        id: user_id,
                        username: username.to_string(),
                        role,
                        created_at: None,
                        first_login: first_login.unwrap_or(0) == 1,
                    }),
                    message: None,
                }
            } else {
                LoginResponse {
                    success: false,
                    token: None,
                    user: None,
                    message: Some("密码错误".to_string()),
                }
            }
        }
        Err(_) => LoginResponse {
            success: false,
            token: None,
            user: None,
            message: Some("用户不存在".to_string()),
        },
    }
}

/// Legacy SHA-256 hashing (only for migration purposes)
fn hash_password_legacy(password: &str, salt: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut data = format!("{}{}", password, salt).into_bytes();
    for _ in 0..10000 {
        let hash = Sha256::digest(&data);
        data = hash.to_vec();
    }
    hex::encode(data)
}

#[allow(dead_code)]
pub fn register_user(username: String, password: String, role: String) -> Result<User, String> {
    let conn = DB.lock();
    
    let existing: Result<i64, _> = conn.query_row(
        "SELECT id FROM users WHERE username = ?1",
        params![username],
        |r| r.get(0),
    );
    if existing.is_ok() {
        return Err("用户名已存在".to_string());
    }

    let bcrypt = hash_password(&password);

    conn.execute(
        "INSERT INTO users (username, bcrypt_hash, role) VALUES (?1, ?2, ?3)",
        params![username, bcrypt, role],
    ).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    Ok(User {
        id,
        username,
        role,
        created_at: Some(chrono::Utc::now().to_rfc3339()),
        first_login: false,
    })
}

pub fn change_password(user_id: i64, old_password: &str, new_password: &str) -> Result<(), String> {
    let conn = DB.lock();
    
    let bcrypt_hash: Option<String> = conn.query_row(
        "SELECT bcrypt_hash FROM users WHERE id = ?1",
        params![user_id],
        |r| r.get(0),
    ).ok();

    let valid = if let Some(ref bh) = bcrypt_hash {
        verify_password(old_password, bh)
    } else {
        return Err("无法修改密码：请使用重置功能".to_string());
    };

    if !valid {
        return Err("旧密码错误".to_string());
    }

    let new_hash = hash_password(new_password);
    conn.execute(
        "UPDATE users SET bcrypt_hash = ?1, first_login = 0 WHERE id = ?2",
        params![new_hash, user_id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn setup_first_login(user_id: i64, username: &str, new_password: &str) -> Result<(), String> {
    let conn = DB.lock();
    let new_hash = hash_password(new_password);
    conn.execute(
        "UPDATE users SET username = ?1, bcrypt_hash = ?2, first_login = 0 WHERE id = ?3",
        params![username, new_hash, user_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[allow(dead_code)]
pub fn verify_token(token: &str) -> Option<i64> {
    let conn = DB.lock();
    
    // Check if token is revoked
    let revoked: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM revoked_tokens WHERE token = ?1",
        params![token],
        |r| r.get(0),
    );
    if revoked.unwrap_or(0) > 0 {
        return None;
    }
    
    // Verify token and check expiry
    let result: Option<i64> = conn.query_row(
        "SELECT user_id FROM user_tokens WHERE token = ?1 AND expires_at > datetime('now')",
        params![token],
        |r| r.get(0),
    ).ok();
    
    result
}

pub fn revoke_token(token: &str) {
    let conn = DB.lock();
    conn.execute(
        "INSERT OR IGNORE INTO revoked_tokens (token) VALUES (?1)",
        params![token],
    ).ok();
}
