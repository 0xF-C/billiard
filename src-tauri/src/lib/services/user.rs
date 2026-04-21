use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::hash_password;
use rusqlite::params;
use log::error;

pub fn create_user(username: String, password: String, role: String) -> Result<User, String> {
    let conn = DB.lock();
    let existing: Result<i64, _> = conn.query_row("SELECT id FROM users WHERE username = ?1", params![username], |r| r.get(0));
    if existing.is_ok() { return Err("用户名已存在".to_string()); }
    let bcrypt = hash_password(&password);
    conn.execute("INSERT INTO users (username, bcrypt_hash, role) VALUES (?1, ?2, ?3)", params![username, bcrypt, role]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(User { id, username, role, created_at: Some(chrono::Utc::now().to_rfc3339()), first_login: false })
}

pub fn get_users() -> Vec<User> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, username, role, created_at FROM users") {
        Ok(s) => s, Err(e) => { error!("Failed to prepare users query: {}", e); return vec![]; }
    };
    let result: Vec<User> = match stmt.query_map([], |row| {
        Ok(User { id: row.get(0)?, username: row.get(1)?, role: row.get(2)?, created_at: row.get(3)?, first_login: false })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(e) => { error!("Failed to query users: {}", e); vec![] } };
    result
}

pub fn update_user(id: i64, role: String) -> Result<User, String> {
    let conn = DB.lock();
    conn.execute("UPDATE users SET role = ?1 WHERE id = ?2", params![role, id]).map_err(|e| e.to_string())?;
    conn.query_row("SELECT id, username, role, created_at FROM users WHERE id = ?1", params![id], |row| {
        Ok(User { id: row.get(0)?, username: row.get(1)?, role: row.get(2)?, created_at: row.get(3)?, first_login: false })
    }).map_err(|e| e.to_string())
}

pub fn delete_user(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM users WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
