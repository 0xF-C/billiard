use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;

pub fn get_inventory_categories() -> Vec<InventoryCategory> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, description, created_at FROM inventory_categories ORDER BY id") { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<InventoryCategory> = match stmt.query_map([], |row| { Ok(InventoryCategory { id: row.get(0)?, name: row.get(1)?, description: row.get(2)?, created_at: row.get(3)? }) }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_inventory_category(name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    conn.execute("INSERT INTO inventory_categories (name, description) VALUES (?1, ?2)", params![name, description]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({ "id": id, "name": name, "description": description }))
}

pub fn update_inventory_category(id: i64, name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    conn.execute("UPDATE inventory_categories SET name = ?1, description = ?2 WHERE id = ?3", params![name, description, id]).map_err(|e| e.to_string())?;
    Ok(serde_json::json!({ "id": id, "name": name, "description": description }))
}

pub fn delete_inventory_category(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM inventory_categories WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_inventory(category_id: Option<i64>, search: Option<String>) -> Vec<InventoryItem> {
    let conn = DB.lock();
    let query = "SELECT i.id, i.name, i.category_id, COALESCE(c.name, ''), i.quantity, i.price, COALESCE(i.cost,0), i.unit, i.supplier, COALESCE(i.alert_stock,0), i.created_at FROM inventory i LEFT JOIN inventory_categories c ON i.category_id = c.id WHERE (?1 IS NULL OR i.category_id = ?1) AND (?2 IS NULL OR i.name LIKE '%' || ?2 || '%') ORDER BY i.id DESC";
    let mut stmt = match conn.prepare(query) { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<InventoryItem> = match stmt.query_map(params![category_id, search], |row| {
        Ok(InventoryItem { id: row.get(0)?, name: row.get(1)?, category_id: row.get(2)?, category_name: row.get(3)?, quantity: row.get(4)?, price: row.get(5)?, cost: row.get(6)?, unit: row.get(7)?, supplier: row.get(8)?, alert_stock: row.get(9)?, created_at: row.get(10)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_inventory(req: CreateInventoryRequest) -> Result<InventoryItem, String> {
    let conn = DB.lock();
    let unit_val = req.unit.clone().unwrap_or_else(|| "个".to_string());
    conn.execute("INSERT INTO inventory (name, category_id, quantity, price, cost, unit, supplier, alert_stock) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![req.name, req.category_id, req.quantity, req.price, req.cost.unwrap_or(0.0), unit_val, req.supplier, req.alert_stock.unwrap_or(0)]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    let category_name: Option<String> = if let Some(cid) = req.category_id { conn.query_row("SELECT name FROM inventory_categories WHERE id = ?1", params![cid], |r| r.get(0)).ok() } else { None };
    Ok(InventoryItem { id, name: req.name, category_id: req.category_id, category_name, quantity: req.quantity, price: req.price, cost: req.cost.unwrap_or(0.0), unit: unit_val, supplier: req.supplier, alert_stock: req.alert_stock.unwrap_or(0), created_at: Some(chrono::Utc::now().to_rfc3339()) })
}

pub fn update_inventory(id: i64, req: UpdateInventoryRequest) -> Result<InventoryItem, String> {
    let conn = DB.lock();
    if let Some(ref name) = req.name { conn.execute("UPDATE inventory SET name = ?1 WHERE id = ?2", params![name, id]).ok(); }
    if let Some(cid) = req.category_id { conn.execute("UPDATE inventory SET category_id = ?1 WHERE id = ?2", params![cid, id]).ok(); }
    if let Some(qty) = req.quantity { conn.execute("UPDATE inventory SET quantity = ?1 WHERE id = ?2", params![qty, id]).ok(); }
    if let Some(price) = req.price { conn.execute("UPDATE inventory SET price = ?1 WHERE id = ?2", params![price, id]).ok(); }
    if let Some(cost) = req.cost { conn.execute("UPDATE inventory SET cost = ?1 WHERE id = ?2", params![cost, id]).ok(); }
    if let Some(ref unit) = req.unit { conn.execute("UPDATE inventory SET unit = ?1 WHERE id = ?2", params![unit, id]).ok(); }
    if let Some(ref sup) = req.supplier { conn.execute("UPDATE inventory SET supplier = ?1 WHERE id = ?2", params![sup, id]).ok(); }
    if let Some(alert) = req.alert_stock { conn.execute("UPDATE inventory SET alert_stock = ?1 WHERE id = ?2", params![alert, id]).ok(); }
    drop(conn);
    get_inventory(None, None).into_iter().find(|i| i.id == id).ok_or("库存项不存在".to_string())
}

pub fn delete_inventory(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM inventory WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
