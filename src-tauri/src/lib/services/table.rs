use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;
use log::error;

pub fn get_tables(include_order: bool) -> Vec<Table> {
    let conn = DB.lock();
    let mut stmt = match conn
        .prepare("SELECT id, name, table_type, status, rate_per_hour, is_private, min_hours, is_active, area_id FROM tables WHERE is_active = 1")
    {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to prepare tables query: {}", e);
            return vec![];
        }
    };

    let tables: Vec<Table> = match stmt.query_map([], |row| {
        let table_id: i64 = row.get(0)?;
        let mut table = Table {
            id: table_id,
            name: row.get(1)?,
            table_type: row.get(2)?,
            status: row.get(3)?,
            rate_per_hour: row.get(4)?,
            is_private: row.get::<_, i32>(5)? != 0,
            min_hours: row.get(6)?,
            is_active: row.get::<_, i32>(7)? != 0,
            area_id: row.get(8)?,
            current_order: None,
        };

        if include_order && table.status == "使用中" {
            let order_result: Result<(i64, String, String), _> = conn.query_row(
                "SELECT o.id, o.start_time, COALESCE(m.name, o.customer_name, '散客') as member_name 
                 FROM orders o 
                 LEFT JOIN members m ON o.member_id = m.id 
                 WHERE o.table_id = ?1 AND o.status = '进行中'",
                params![table_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            );
            if let Ok((order_id, start_time, member_name)) = order_result {
                table.current_order = Some(CurrentOrder {
                    id: order_id,
                    start_time,
                    member_name,
                });
            }
        }

        Ok(table)
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => {
            error!("Failed to query tables: {}", e);
            vec![]
        }
    };

    tables
}

pub fn create_table(name: String, table_type: String, rate_per_hour: f64, is_private: bool, area_id: Option<i64>) -> Result<Table, String> {
    let conn = DB.lock();
    conn.execute(
        "INSERT INTO tables (name, table_type, rate_per_hour, is_private, area_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, table_type, rate_per_hour, is_private as i32, area_id],
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    drop(conn);
    
    get_tables(false).into_iter().find(|t| t.id == id).ok_or("创建失败".to_string())
}

pub fn update_table(id: i64, data: serde_json::Value) -> Result<Table, String> {
    let conn = DB.lock();
    
    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let table_type = data.get("table_type").and_then(|v| v.as_str()).unwrap_or("普通");
    let rate_per_hour = data.get("rate_per_hour").and_then(|v| v.as_f64()).unwrap_or(30.0);
    let is_private = data.get("is_private").and_then(|v| v.as_bool()).unwrap_or(false);
    let min_hours = data.get("min_hours").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let is_active = data.get("is_active").and_then(|v| v.as_bool()).unwrap_or(true);
    let area_id = data.get("area_id").and_then(|v| v.as_i64());

    conn.execute(
        "UPDATE tables SET name = ?1, table_type = ?2, rate_per_hour = ?3, is_private = ?4, 
         min_hours = ?5, is_active = ?6, area_id = ?7 WHERE id = ?8",
        params![name, table_type, rate_per_hour, is_private as i32, min_hours, is_active as i32, area_id, id],
    ).map_err(|e| e.to_string())?;

    drop(conn);
    get_tables(false).into_iter().find(|t| t.id == id).ok_or("球桌不存在".to_string())
}

pub fn delete_table(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("UPDATE tables SET is_active = 0 WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_table_categories() -> Vec<serde_json::Value> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, description, created_at FROM table_categories") {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to prepare table categories query: {}", e);
            return vec![];
        }
    };
    let result: Vec<serde_json::Value> = match stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "description": row.get::<_, Option<String>>(2)?,
            "created_at": row.get::<_, Option<String>>(3)?,
        }))
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => {
            error!("Failed to query table categories: {}", e);
            vec![]
        }
    };
    result
}

pub fn create_table_category(name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    conn.execute(
        "INSERT INTO table_categories (name, description) VALUES (?1, ?2)",
        params![name, description],
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({
        "id": id,
        "name": name,
        "description": description,
    }))
}

pub fn update_table_category(id: i64, name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    conn.execute("UPDATE table_categories SET name = ?1, description = ?2 WHERE id = ?3", params![name, description, id])
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({ "id": id, "name": name, "description": description }))
}

pub fn delete_table_category(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM table_categories WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
