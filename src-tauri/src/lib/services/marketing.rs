// 营销服务模块 - 库存出入库、积分、优惠券、短信营销

use crate::lib::db::DB;
use log::error;
use rusqlite::params;

// ======== 库存出入库 ========

pub fn get_stock_io_records(product_id: Option<i64>, io_type: Option<String>, days: Option<i32>) -> Vec<serde_json::Value> {
    let conn = DB.lock();
    
    let mut query = String::from(
        "SELECT r.id, r.product_id, p.name as product_name, r.type, r.quantity, 
         r.before_stock, r.after_stock, r.unit_price, r.total_price, 
         r.reason, r.remark, r.operator_id, r.operator, r.created_at
         FROM stock_io_records r
         LEFT JOIN inventory p ON r.product_id = p.id WHERE 1=1"
    );
    
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(pid) = product_id {
        query.push_str(" AND r.product_id = ?");
        params_vec.push(Box::new(pid));
    }
    if let Some(t) = io_type {
        query.push_str(" AND r.type = ?");
        params_vec.push(Box::new(t));
    }
    if let Some(d) = days {
        query.push_str(&format!(" AND r.created_at >= datetime('now', '-{} days')", d));
    }
    query.push_str(" ORDER BY r.id DESC LIMIT 200");
    
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    
    let mut stmt = match conn.prepare(&query) {
        Ok(s) => s,
        Err(e) => { error!("Prepare error: {}", e); return vec![] }
    };
    
    let result: Vec<serde_json::Value> = match stmt.query_map(params_refs.as_slice(), |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "product_id": row.get::<_, i64>(1)?,
            "product_name": row.get::<_, Option<String>>(2)?,
            "type": row.get::<_, String>(3)?,
            "quantity": row.get::<_, i64>(4)?,
            "before_stock": row.get::<_, i64>(5)?,
            "after_stock": row.get::<_, i64>(6)?,
            "unit_price": row.get::<_, f64>(7).unwrap_or(0.0),
            "total_price": row.get::<_, f64>(8).unwrap_or(0.0),
            "reason": row.get::<_, Option<String>>(9)?,
            "remark": row.get::<_, Option<String>>(10)?,
            "operator_id": row.get::<_, Option<i64>>(11)?,
            "operator": row.get::<_, Option<String>>(12)?,
            "created_at": row.get::<_, String>(13)?
        }))
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => { error!("Query error: {}", e); vec![] }
    };
    result
}

pub fn create_stock_in(product_id: i64, quantity: i64, reason: Option<String>, remark: Option<String>, unit_price: Option<f64>, operator_id: Option<i64>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    let before_stock: i64 = conn.query_row(
        "SELECT COALESCE(quantity, 0) FROM inventory WHERE id = ?1",
        params![product_id],
        |r| r.get(0)
    ).unwrap_or(0);
    
    let after_stock = before_stock + quantity;
    let unit_price = unit_price.unwrap_or(0.0);
    let total_price = unit_price * quantity as f64;
    
    conn.execute(
        "UPDATE inventory SET quantity = quantity + ?1 WHERE id = ?2",
        params![quantity, product_id]
    ).ok();
    
    conn.execute(
        "INSERT INTO stock_io_records (product_id, type, quantity, before_stock, after_stock, unit_price, total_price, reason, remark, operator_id, created_at)
         VALUES (?1, 'in', ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
        params![product_id, quantity, before_stock, after_stock, unit_price, total_price, reason, remark, operator_id]
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({"id": id, "before_stock": before_stock, "after_stock": after_stock}))
}

pub fn create_stock_out(product_id: i64, quantity: i64, reason: Option<String>, remark: Option<String>, operator_id: Option<i64>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    let before_stock: i64 = conn.query_row(
        "SELECT COALESCE(quantity, 0) FROM inventory WHERE id = ?1",
        params![product_id],
        |r| r.get(0)
    ).unwrap_or(0);
    
    if before_stock < quantity {
        return Err("库存不足".to_string());
    }
    
    let after_stock = before_stock - quantity;
    
    conn.execute(
        "UPDATE inventory SET quantity = quantity - ?1 WHERE id = ?2",
        params![quantity, product_id]
    ).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO stock_io_records (product_id, type, quantity, before_stock, after_stock, reason, remark, operator_id, created_at)
         VALUES (?1, 'out', ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))",
        params![product_id, quantity, before_stock, after_stock, reason, remark, operator_id]
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({"id": id, "before_stock": before_stock, "after_stock": after_stock}))
}

// ======== 积分管理 ========

pub fn get_points_logs(member_id: Option<i64>, days: Option<i32>) -> Vec<serde_json::Value> {
    let conn = DB.lock();
    
    let mut query = String::from(
        "SELECT l.id, l.member_id, m.name as member_name, m.phone, l.points, l.type, l.order_id, l.remark, l.operator, l.created_at
         FROM points_logs l
         LEFT JOIN members m ON l.member_id = m.id WHERE 1=1"
    );
    
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(mid) = member_id {
        query.push_str(" AND l.member_id = ?");
        params_vec.push(Box::new(mid));
    }
    if let Some(d) = days {
        query.push_str(&format!(" AND l.created_at >= datetime('now', '-{} days')", d));
    }
    query.push_str(" ORDER BY l.id DESC LIMIT 200");
    
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    
    let mut stmt = match conn.prepare(&query) {
        Ok(s) => s,
        Err(e) => { error!("Prepare error: {}", e); return vec![] }
    };
    
    let result: Vec<serde_json::Value> = match stmt.query_map(params_refs.as_slice(), |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "member_id": row.get::<_, i64>(1)?,
            "member_name": row.get::<_, Option<String>>(2)?,
            "phone": row.get::<_, Option<String>>(3)?,
            "points": row.get::<_, i64>(4)?,
            "type": row.get::<_, String>(5)?,
            "order_id": row.get::<_, Option<i64>>(6)?,
            "remark": row.get::<_, Option<String>>(7)?,
            "operator": row.get::<_, Option<String>>(8)?,
            "created_at": row.get::<_, String>(9)?
        }))
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => { error!("Query error: {}", e); vec![] }
    };
    result
}

pub fn add_points(member_id: i64, points: i64, points_type: Option<String>, remark: Option<String>, order_id: Option<i64>, operator_id: Option<i64>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    let pts_type = points_type.unwrap_or_else(|| "earn".to_string());
    let operator = conn.query_row(
        "SELECT username FROM users WHERE id = ?1",
        params![operator_id],
        |r| r.get::<_, String>(0)
    ).ok();
    
    conn.execute(
        "INSERT INTO points_logs (member_id, points, type, remark, order_id, operator, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))",
        params![member_id, points, pts_type, remark, order_id, operator]
    ).map_err(|e| e.to_string())?;
    
    if pts_type == "earn" {
        conn.execute(
            "UPDATE members SET points = COALESCE(points, 0) + ?1 WHERE id = ?2",
            params![points, member_id]
        ).ok();
    } else {
        conn.execute(
            "UPDATE members SET points = COALESCE(points, 0) - ?1 WHERE id = ?2",
            params![points, member_id]
        ).ok();
    }
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({"id": id, "member_id": member_id, "points": points, "type": pts_type}))
}

// ======== 优惠券管理 ========

pub fn get_coupons(status: Option<String>) -> Vec<serde_json::Value> {
    let conn = DB.lock();
    
    let query = if status.is_some() {
        "SELECT id, name, coupon_type, discount, min_amount, quantity, used_count, valid_from, valid_to, status, description, created_at FROM coupons WHERE status = ?1 ORDER BY id DESC"
    } else {
        "SELECT id, name, coupon_type, discount, min_amount, quantity, used_count, valid_from, valid_to, status, description, created_at FROM coupons ORDER BY id DESC"
    };
    
    let mut stmt = match conn.prepare(query) {
        Ok(s) => s,
        Err(e) => { error!("Prepare error: {}", e); return vec![] }
    };
    
    let result: Vec<serde_json::Value> = if let Some(s) = status {
        match stmt.query_map(params![s], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "coupon_type": row.get::<_, String>(2)?,
                "discount": row.get::<_, f64>(3)?,
                "min_amount": row.get::<_, f64>(4)?,
                "quantity": row.get::<_, i64>(5)?,
                "used_count": row.get::<_, i64>(6)?,
                "valid_from": row.get::<_, Option<String>>(7)?,
                "valid_to": row.get::<_, Option<String>>(8)?,
                "status": row.get::<_, String>(9)?,
                "description": row.get::<_, Option<String>>(10)?,
                "created_at": row.get::<_, String>(11)?
            }))
        }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(e) => { error!("Query error: {}", e); vec![] } }
    } else {
        match stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "coupon_type": row.get::<_, String>(2)?,
                "discount": row.get::<_, f64>(3)?,
                "min_amount": row.get::<_, f64>(4)?,
                "quantity": row.get::<_, i64>(5)?,
                "used_count": row.get::<_, i64>(6)?,
                "valid_from": row.get::<_, Option<String>>(7)?,
                "valid_to": row.get::<_, Option<String>>(8)?,
                "status": row.get::<_, String>(9)?,
                "description": row.get::<_, Option<String>>(10)?,
                "created_at": row.get::<_, String>(11)?
            }))
        }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(e) => { error!("Query error: {}", e); vec![] } }
    };
    result
}

pub fn create_coupon(name: String, coupon_type: Option<String>, discount: f64, min_amount: Option<f64>, quantity: i64, valid_from: Option<String>, valid_to: Option<String>, status: Option<String>, description: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    conn.execute(
        "INSERT INTO coupons (name, coupon_type, discount, min_amount, quantity, valid_from, valid_to, status, description, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
        params![
            name,
            coupon_type.unwrap_or_else(|| "cash".to_string()),
            discount,
            min_amount.unwrap_or(0.0),
            quantity,
            valid_from,
            valid_to,
            status.unwrap_or_else(|| "active".to_string()),
            description
        ]
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({"id": id, "name": name}))
}

pub fn update_coupon(id: i64, name: String, coupon_type: Option<String>, discount: f64, min_amount: Option<f64>, quantity: i64, valid_from: Option<String>, valid_to: Option<String>, status: Option<String>, description: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    conn.execute(
        "UPDATE coupons SET name = ?1, coupon_type = ?2, discount = ?3, min_amount = ?4, quantity = ?5, valid_from = ?6, valid_to = ?7, status = ?8, description = ?9 WHERE id = ?10",
        params![
            name,
            coupon_type.unwrap_or_else(|| "cash".to_string()),
            discount,
            min_amount.unwrap_or(0.0),
            quantity,
            valid_from,
            valid_to,
            status.unwrap_or_else(|| "active".to_string()),
            description,
            id
        ]
    ).map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({"id": id}))
}

pub fn delete_coupon(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM coupon_claims WHERE coupon_id = ?1", params![id]).ok();
    conn.execute("DELETE FROM coupons WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn claim_coupon(coupon_id: i64, member_id: i64) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    let member_name: Option<String> = conn.query_row(
        "SELECT name FROM members WHERE id = ?1",
        params![member_id],
        |r| r.get(0)
    ).ok();
    
    conn.execute(
        "INSERT INTO coupon_claims (coupon_id, member_id, member_name, status, created_at)
         VALUES (?1, ?2, ?3, 'unused', datetime('now'))",
        params![coupon_id, member_id, member_name]
    ).map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE coupons SET used_count = used_count + 1 WHERE id = ?1",
        params![coupon_id]
    ).ok();
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({"id": id, "coupon_id": coupon_id, "member_id": member_id}))
}

// ======== 短信营销 ========

pub fn get_sms_templates() -> Vec<serde_json::Value> {
    let conn = DB.lock();
    
    let mut stmt = match conn.prepare("SELECT id, name, content, template_type, usage_count, created_at FROM sms_templates ORDER BY id DESC") {
        Ok(s) => s,
        Err(e) => { error!("Prepare error: {}", e); return vec![] }
    };
    
    let result: Vec<serde_json::Value> = match stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "content": row.get::<_, String>(2)?,
            "template_type": row.get::<_, String>(3)?,
            "usage_count": row.get::<_, i64>(4)?,
            "created_at": row.get::<_, String>(5)?
        }))
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => { error!("Query error: {}", e); vec![] }
    };
    result
}

pub fn create_sms_template(name: String, content: String, template_type: Option<String>) -> Result<serde_json::Value, String> {
    let conn = DB.lock();
    
    conn.execute(
        "INSERT INTO sms_templates (name, content, template_type, created_at) VALUES (?1, ?2, ?3, datetime('now'))",
        params![name, content, template_type.unwrap_or_else(|| "notification".to_string())]
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    Ok(serde_json::json!({"id": id, "name": name}))
}

pub fn delete_sms_template(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM sms_templates WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}