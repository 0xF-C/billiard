use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::services::settings::load_settings;
use rusqlite::params;

pub fn get_packages() -> Vec<PackageEntity> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, price, hours, area_ids, table_ids, start_time, end_time, is_active, created_at FROM packages ORDER BY id") {
        Ok(s) => s,
        Err(_) => {
            let settings = load_settings();
            return settings.packages.iter().map(|p| PackageEntity {
                id: p.id as i64, name: p.name.clone(), price: p.price, hours: p.hours,
                area_ids: p.area_ids.clone(), table_ids: p.table_ids.clone(),
                start_time: p.start_time.clone(), end_time: p.end_time.clone(),
                is_active: true, created_at: None,
            }).collect();
        }
    };
    let result: Vec<PackageEntity> = match stmt.query_map([], |row| {
        Ok(PackageEntity {
            id: row.get(0)?, name: row.get(1)?, price: row.get(2)?, hours: row.get(3)?,
            area_ids: row.get(4)?, table_ids: row.get(5)?, start_time: row.get(6)?,
            end_time: row.get(7)?, is_active: row.get::<_, i32>(8)? != 0, created_at: row.get(9)?,
        })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_package(req: CreatePackageRequest) -> Result<PackageEntity, String> {
    let conn = DB.lock();
    conn.execute("INSERT INTO packages (name, price, hours, area_ids, table_ids, start_time, end_time, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)",
        params![req.name, req.price, req.hours, req.area_ids, req.table_ids, req.start_time, req.end_time]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(PackageEntity { id, name: req.name, price: req.price, hours: req.hours, area_ids: req.area_ids, table_ids: req.table_ids, start_time: req.start_time, end_time: req.end_time, is_active: true, created_at: Some(chrono::Utc::now().to_rfc3339()) })
}

pub fn update_package(id: i64, req: CreatePackageRequest) -> Result<PackageEntity, String> {
    let conn = DB.lock();
    conn.execute("UPDATE packages SET name=?1, price=?2, hours=?3, area_ids=?4, table_ids=?5, start_time=?6, end_time=?7 WHERE id=?8",
        params![req.name, req.price, req.hours, req.area_ids, req.table_ids, req.start_time, req.end_time, id]).map_err(|e| e.to_string())?;
    Ok(PackageEntity { id, name: req.name, price: req.price, hours: req.hours, area_ids: req.area_ids, table_ids: req.table_ids, start_time: req.start_time, end_time: req.end_time, is_active: true, created_at: None })
}

pub fn delete_package(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM packages WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
