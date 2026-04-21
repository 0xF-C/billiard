use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;

pub fn get_product_categories() -> Vec<ProductCategory> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, description, created_at FROM product_categories ORDER BY id") { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<ProductCategory> = match stmt.query_map([], |row| { Ok(ProductCategory { id: row.get(0)?, name: row.get(1)?, description: row.get(2)?, created_at: row.get(3)? }) }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_product_category(name: String, description: Option<String>) -> Result<ProductCategory, String> {
    let conn = DB.lock();
    conn.execute("INSERT INTO product_categories (name, description) VALUES (?1, ?2)", params![name, description]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(ProductCategory { id, name, description, created_at: Some(chrono::Utc::now().to_rfc3339()) })
}

pub fn update_product_category(id: i64, name: String, description: Option<String>) -> Result<ProductCategory, String> {
    let conn = DB.lock();
    conn.execute("UPDATE product_categories SET name = ?1, description = ?2 WHERE id = ?3", params![name, description, id]).map_err(|e| e.to_string())?;
    Ok(ProductCategory { id, name, description, created_at: None })
}

pub fn delete_product_category(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM product_categories WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_products(category_id: Option<i64>, search: Option<String>) -> Vec<Product> {
    let conn = DB.lock();
    let query = "SELECT p.id, p.name, p.category_id, COALESCE(c.name, ''), p.price, COALESCE(p.cost,0), p.unit, p.stock, COALESCE(p.alert_stock,0), p.image, p.description, p.is_active, p.created_at FROM products p LEFT JOIN product_categories c ON p.category_id = c.id WHERE (?1 IS NULL OR p.category_id = ?1) AND (?2 IS NULL OR p.name LIKE '%' || ?2 || '%') ORDER BY p.id DESC";
    let mut stmt = match conn.prepare(query) { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<Product> = match stmt.query_map(params![category_id, search], |row| {
        Ok(Product { id: row.get(0)?, name: row.get(1)?, category_id: row.get(2)?, category_name: row.get(3)?, price: row.get(4)?, cost: row.get(5)?, unit: row.get(6)?, stock: row.get(7)?, alert_stock: row.get(8)?, image: row.get(9)?, description: row.get(10)?, is_active: row.get::<_, i32>(11)? != 0, created_at: row.get(12)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_product(req: CreateProductRequest) -> Result<Product, String> {
    let conn = DB.lock();
    let unit_val = req.unit.clone().unwrap_or_else(|| "个".to_string());
    conn.execute("INSERT INTO products (name, category_id, price, cost, unit, stock, alert_stock, image, description, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 1)", params![req.name, req.category_id, req.price, req.cost.unwrap_or(0.0), unit_val, req.stock.unwrap_or(0), req.alert_stock.unwrap_or(0), req.image, req.description]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    let category_name: Option<String> = if let Some(cid) = req.category_id { conn.query_row("SELECT name FROM product_categories WHERE id = ?1", params![cid], |r| r.get(0)).ok() } else { None };
    Ok(Product { id, name: req.name, category_id: req.category_id, category_name, price: req.price, cost: req.cost.unwrap_or(0.0), unit: unit_val, stock: req.stock.unwrap_or(0), alert_stock: req.alert_stock.unwrap_or(0), image: req.image, description: req.description, is_active: true, created_at: Some(chrono::Utc::now().to_rfc3339()) })
}

pub fn update_product(id: i64, req: UpdateProductRequest) -> Result<Product, String> {
    let conn = DB.lock();
    if let Some(ref name) = req.name { conn.execute("UPDATE products SET name = ?1 WHERE id = ?2", params![name, id]).ok(); }
    if let Some(cid) = req.category_id { conn.execute("UPDATE products SET category_id = ?1 WHERE id = ?2", params![cid, id]).ok(); }
    if let Some(price) = req.price { conn.execute("UPDATE products SET price = ?1 WHERE id = ?2", params![price, id]).ok(); }
    if let Some(cost) = req.cost { conn.execute("UPDATE products SET cost = ?1 WHERE id = ?2", params![cost, id]).ok(); }
    if let Some(ref unit) = req.unit { conn.execute("UPDATE products SET unit = ?1 WHERE id = ?2", params![unit, id]).ok(); }
    if let Some(stock) = req.stock { conn.execute("UPDATE products SET stock = ?1 WHERE id = ?2", params![stock, id]).ok(); }
    if let Some(alert) = req.alert_stock { conn.execute("UPDATE products SET alert_stock = ?1 WHERE id = ?2", params![alert, id]).ok(); }
    if let Some(ref img) = req.image { conn.execute("UPDATE products SET image = ?1 WHERE id = ?2", params![img, id]).ok(); }
    if let Some(ref desc) = req.description { conn.execute("UPDATE products SET description = ?1 WHERE id = ?2", params![desc, id]).ok(); }
    if let Some(active) = req.is_active { conn.execute("UPDATE products SET is_active = ?1 WHERE id = ?2", params![active as i32, id]).ok(); }
    drop(conn);
    get_products(None, None).into_iter().find(|p| p.id == id).ok_or("产品不存在".to_string())
}

pub fn delete_product(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM products WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
