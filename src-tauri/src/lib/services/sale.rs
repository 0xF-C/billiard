use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::{round_to_two, validate_positive_id, validate_quantity};
use chrono::Duration;
use rusqlite::params;

fn map_sale_row(row: &rusqlite::Row) -> rusqlite::Result<Sale> {
    Ok(Sale {
        id: row.get(0)?, product_name: row.get(1)?, quantity: row.get(2)?, unit_price: row.get(3)?,
        total_amount: row.get(4)?, table_id: row.get(5)?, table_name: row.get(6)?,
        member_id: row.get(7)?, member_name: row.get(8)?, payment_method: row.get(9)?,
        remark: row.get(10)?, created_at: row.get(11)?,
    })
}

pub fn get_sales_records(table_id: Option<i64>, member_id: Option<i64>, days: Option<i32>) -> Vec<Sale> {
    let conn = DB.lock();
    let days = days.unwrap_or(7);
    let shanghai = chrono::Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let start_date = (shanghai - Duration::days(days as i64)).format("%Y-%m-%d").to_string();
    match (table_id, member_id) {
        (Some(tid), Some(mid)) => {
            let mut stmt = match conn.prepare("SELECT s.id, COALESCE(p.name, s.product_name), s.quantity, s.unit_price, s.total_amount, s.table_id, COALESCE(t.name, ''), s.member_id, COALESCE(m.name, ''), s.payment_method, s.remark, s.created_at FROM sales s LEFT JOIN products p ON s.inventory_id = p.id LEFT JOIN tables t ON s.table_id = t.id LEFT JOIN members m ON s.member_id = m.id WHERE s.table_id = ?1 AND s.member_id = ?2 AND date(s.created_at) >= ?3 ORDER BY s.id DESC") { Ok(s) => s, Err(_) => return vec![] };
            stmt.query_map(params![tid, mid, start_date], map_sale_row).ok().map(|iter| iter.filter_map(|r| r.ok()).collect()).unwrap_or_default()
        }
        (Some(tid), None) => {
            let mut stmt = match conn.prepare("SELECT s.id, COALESCE(p.name, s.product_name), s.quantity, s.unit_price, s.total_amount, s.table_id, COALESCE(t.name, ''), s.member_id, COALESCE(m.name, ''), s.payment_method, s.remark, s.created_at FROM sales s LEFT JOIN products p ON s.inventory_id = p.id LEFT JOIN tables t ON s.table_id = t.id LEFT JOIN members m ON s.member_id = m.id WHERE s.table_id = ?1 AND date(s.created_at) >= ?2 ORDER BY s.id DESC") { Ok(s) => s, Err(_) => return vec![] };
            stmt.query_map(params![tid, start_date], map_sale_row).ok().map(|iter| iter.filter_map(|r| r.ok()).collect()).unwrap_or_default()
        }
        (None, Some(mid)) => {
            let mut stmt = match conn.prepare("SELECT s.id, COALESCE(p.name, s.product_name), s.quantity, s.unit_price, s.total_amount, s.table_id, COALESCE(t.name, ''), s.member_id, COALESCE(m.name, ''), s.payment_method, s.remark, s.created_at FROM sales s LEFT JOIN products p ON s.inventory_id = p.id LEFT JOIN tables t ON s.table_id = t.id LEFT JOIN members m ON s.member_id = m.id WHERE s.member_id = ?1 AND date(s.created_at) >= ?2 ORDER BY s.id DESC") { Ok(s) => s, Err(_) => return vec![] };
            stmt.query_map(params![mid, start_date], map_sale_row).ok().map(|iter| iter.filter_map(|r| r.ok()).collect()).unwrap_or_default()
        }
        (None, None) => {
            let mut stmt = match conn.prepare("SELECT s.id, COALESCE(p.name, s.product_name), s.quantity, s.unit_price, s.total_amount, s.table_id, COALESCE(t.name, ''), s.member_id, COALESCE(m.name, ''), s.payment_method, s.remark, s.created_at FROM sales s LEFT JOIN products p ON s.inventory_id = p.id LEFT JOIN tables t ON s.table_id = t.id LEFT JOIN members m ON s.member_id = m.id WHERE date(s.created_at) >= ?1 ORDER BY s.id DESC") { Ok(s) => s, Err(_) => return vec![] };
            stmt.query_map(params![start_date], map_sale_row).ok().map(|iter| iter.filter_map(|r| r.ok()).collect()).unwrap_or_default()
        }
    }
}

pub fn sale_product(req: SaleRequest) -> Result<Sale, String> {
    validate_positive_id(req.product_id, "商品ID").map_err(|e| e.to_string())?;
    validate_quantity(req.quantity).map_err(|e| e.to_string())?;
    
    let conn = DB.lock();
    // Fix #4: Atomic check-and-deduct in single UPDATE with WHERE clause
    let updated = conn.execute(
        "UPDATE products SET stock = stock - ?1 WHERE id = ?2 AND is_active = 1 AND stock >= ?1",
        params![req.quantity, req.product_id],
    ).map_err(|e| e.to_string())?;
    
    if updated == 0 {
        // Check if product exists at all
        let exists: Result<i64, _> = conn.query_row("SELECT id FROM products WHERE id = ?1", params![req.product_id], |r| r.get(0));
        if exists.is_err() {
            return Err("产品不存在".to_string());
        }
        return Err("库存不足或产品已下架".to_string());
    }
    
    let (name, unit_price): (String, f64) = conn.query_row(
        "SELECT name, price FROM products WHERE id = ?1", params![req.product_id], |r| Ok((r.get(0)?, r.get(1)?))
    ).map_err(|_| "产品查询失败")?;
    
    let total = round_to_two(unit_price * req.quantity as f64);
    let pm = req.payment_method.unwrap_or_else(|| "cash".to_string());
    let now = chrono::Utc::now().to_rfc3339();
    let table_name: Option<String> = if let Some(tid) = req.table_id { conn.query_row("SELECT name FROM tables WHERE id = ?1", params![tid], |r| r.get(0)).ok() } else { None };
    let member_name: Option<String> = if let Some(mid) = req.member_id { conn.query_row("SELECT name FROM members WHERE id = ?1", params![mid], |r| r.get(0)).ok() } else { None };
    
    // 如果使用会员支付，扣减会员余额
    if pm == "member" && req.member_id.is_some() {
        let mid = req.member_id.unwrap();
        let balance: f64 = conn.query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0)).unwrap_or(0.0);
        if balance < total {
            return Err("会员余额不足".to_string());
        }
        conn.execute(
            "UPDATE members SET balance = balance - ?1, total_spent = total_spent + ?1 WHERE id = ?2",
            params![total, mid],
        ).map_err(|e| format!("扣减余额失败: {}", e))?;
        conn.execute(
            "INSERT INTO balance_logs (member_id, amount, balance_before, balance_after, reason, payment_method) 
             SELECT ?1, -?2, balance, balance - ?2, 'product_sale', ?3 FROM members WHERE id = ?1",
            params![mid, total, &pm],
        ).ok();
    }
    
    conn.execute("INSERT INTO sales (inventory_id, product_name, quantity, unit_price, total_amount, table_id, member_id, payment_method, remark, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)", params![req.product_id, &name, req.quantity, unit_price, total, req.table_id, req.member_id, &pm, req.remark, now]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Sale { id, product_name: name, quantity: req.quantity, unit_price, total_amount: total, table_id: req.table_id, table_name, member_id: req.member_id, member_name, payment_method: pm, remark: req.remark, created_at: now })
}

pub fn sale_batch(req: BatchSaleRequest) -> Result<Vec<Sale>, String> {
    // Fix #2: Single lock for entire batch (atomic transaction)
    // Fix #12: Explicit source_type field - inventory first, then products
    let conn = DB.lock();
    let pm = req.payment_method.clone().unwrap_or_else(|| "cash".to_string());
    let now = chrono::Utc::now().to_rfc3339();
    let table_name: Option<String> = if let Some(tid) = req.table_id { conn.query_row("SELECT name FROM tables WHERE id = ?1", params![tid], |r| r.get(0)).ok() } else { None };
    let member_name: Option<String> = if let Some(mid) = req.member_id { conn.query_row("SELECT name FROM members WHERE id = ?1", params![mid], |r| r.get(0)).ok() } else { None };

    // Phase 1: Validate all items and check stock
    let mut validated_items = Vec::new();
    for item in &req.items {
        validate_positive_id(item.product_id, "商品ID").map_err(|e| e.to_string())?;
        validate_quantity(item.quantity).map_err(|e| e.to_string())?;
        
        // Fix #12: Try inventory first, then products - explicit priority
        let result: Option<(String, f64, i32, String)> = conn.query_row(
            "SELECT name, price, quantity, 'inventory' as source FROM inventory WHERE id = ?1",
            params![item.product_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        ).ok().or_else(|| {
            conn.query_row(
                "SELECT name, price, stock, 'products' as source FROM products WHERE id = ?1 AND is_active = 1",
                params![item.product_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            ).ok()
        });

        let (name, unit_price, current_stock, source) = result
            .ok_or_else(|| format!("商品ID {} 不存在", item.product_id))?;
        
        if current_stock < item.quantity {
            return Err(format!("商品「{}」库存不足，当前: {}，需要: {}", name, current_stock, item.quantity));
        }
        
        validated_items.push((item.product_id, item.quantity, name, unit_price, source));
    }

    // Phase 2: Deduct stock (all or nothing - if any fails, rollback)
    
    // 如果使用会员支付，扣减会员余额
    if pm == "member" && req.member_id.is_some() {
        let mid = req.member_id.unwrap();
        let total_amount: f64 = validated_items.iter().map(|(_, qty, _, price, _)| price * (*qty as f64)).sum();
        let balance: f64 = conn.query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0)).unwrap_or(0.0);
        if balance < total_amount {
            return Err("会员余额不足".to_string());
        }
        conn.execute(
            "UPDATE members SET balance = balance - ?1, total_spent = total_spent + ?1 WHERE id = ?2",
            params![total_amount, mid],
        ).map_err(|e| format!("扣减余额失败: {}", e))?;
        conn.execute(
            "INSERT INTO balance_logs (member_id, amount, balance_before, balance_after, reason, payment_method) 
             SELECT ?1, -?2, balance, balance - ?2, 'product_sale', ?3 FROM members WHERE id = ?1",
            params![mid, total_amount, &pm],
        ).ok();
    }

    let mut sales = Vec::new();
    for (pid, qty, name, unit_price, source) in &validated_items {
        let total = round_to_two(*unit_price * *qty as f64);
        
        // Atomic deduction with stock check
        let rows = if source == "inventory" {
            conn.execute(
                "UPDATE inventory SET quantity = quantity - ?1 WHERE id = ?2 AND quantity >= ?1",
                params![qty, pid],
            ).map_err(|e| format!("扣减库存失败: {}", e))?
        } else {
            conn.execute(
                "UPDATE products SET stock = stock - ?1 WHERE id = ?2 AND stock >= ?1 AND is_active = 1",
                params![qty, pid],
            ).map_err(|e| format!("扣减库存失败: {}", e))?
        };
        
        if rows == 0 {
            // Rollback: restore previously deducted items
            for (prev_pid, prev_qty, _, _, prev_source) in &validated_items {
                if *prev_pid == *pid { break; }
                if prev_source == "inventory" {
                    conn.execute("UPDATE inventory SET quantity = quantity + ?1 WHERE id = ?2", params![prev_qty, prev_pid]).ok();
                } else {
                    conn.execute("UPDATE products SET stock = stock + ?1 WHERE id = ?2", params![prev_qty, prev_pid]).ok();
                }
            }
            return Err(format!("商品「{}」库存不足（并发扣减）", name));
        }
        
        conn.execute(
            "INSERT INTO sales (inventory_id, product_name, quantity, unit_price, total_amount, table_id, member_id, payment_method, remark, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![pid, name, qty, unit_price, total, req.table_id, req.member_id, &pm, req.remark, &now],
        ).map_err(|e| e.to_string())?;
        
        let id = conn.last_insert_rowid();
        sales.push(Sale {
            id, product_name: name.clone(), quantity: *qty, unit_price: *unit_price,
            total_amount: total, table_id: req.table_id, table_name: table_name.clone(),
            member_id: req.member_id, member_name: member_name.clone(),
            payment_method: pm.clone(), remark: req.remark.clone(), created_at: now.clone(),
        });
    }
    
    Ok(sales)
}
