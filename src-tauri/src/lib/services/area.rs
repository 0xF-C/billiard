use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;
use log::error;

pub fn get_areas() -> Vec<Area> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT a.id, a.name, a.rate_per_hour, a.created_at, COUNT(t.id) as table_count FROM areas a LEFT JOIN tables t ON t.area_id = a.id AND t.is_active = 1 GROUP BY a.id ORDER BY a.id") {
        Ok(s) => s, Err(e) => { error!("Failed to prepare areas query: {}", e); return vec![]; }
    };
    let result: Vec<Area> = match stmt.query_map([], |row| {
        Ok(Area { id: row.get(0)?, name: row.get(1)?, rate_per_hour: row.get(2)?, created_at: row.get(3)?, table_count: row.get::<_, i64>(4)? as i32 })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(e) => { error!("Failed to query areas: {}", e); vec![] } };
    result
}

pub fn create_area(name: String, rate_per_hour: f64) -> Result<Area, String> {
    let conn = DB.lock();
    conn.execute("INSERT INTO areas (name, rate_per_hour) VALUES (?1, ?2)", params![name, rate_per_hour]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Area { id, name, rate_per_hour, created_at: Some(chrono::Utc::now().to_rfc3339()), table_count: 0 })
}

pub fn update_area(id: i64, name: String, rate_per_hour: f64) -> Result<Area, String> {
    let conn = DB.lock();
    conn.execute("UPDATE areas SET name = ?1, rate_per_hour = ?2 WHERE id = ?3", params![name, rate_per_hour, id]).map_err(|e| e.to_string())?;
    Ok(Area { id, name, rate_per_hour, created_at: None, table_count: 0 })
}

pub fn delete_area(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM areas WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
