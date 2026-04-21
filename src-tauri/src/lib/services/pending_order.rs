use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;

pub fn get_pending_orders() -> Vec<PendingOrder> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT p.id, p.order_no, p.table_id, t.name, p.customer_name, p.customer_phone, p.estimated_amount, p.remark, p.status, p.created_at FROM pending_orders p LEFT JOIN tables t ON p.table_id = t.id WHERE p.status = 'pending' ORDER BY p.created_at DESC") { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<PendingOrder> = match stmt.query_map([], |row| {
        Ok(PendingOrder { id: row.get(0)?, order_no: row.get(1)?, table_id: row.get(2)?, table_name: row.get(3).unwrap_or_default(), customer_name: row.get(4)?, customer_phone: row.get(5)?, estimated_amount: row.get(6).unwrap_or(0.0), remark: row.get(7)?, status: row.get(8)?, created_at: row.get(9)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_pending_order(req: CreatePendingOrderRequest) -> Result<PendingOrder, String> {
    let conn = DB.lock();
    let order_no = format!("P{:06}", conn.last_insert_rowid() + 1000);
    conn.execute("INSERT INTO pending_orders (order_no, table_id, customer_name, customer_phone, estimated_amount, remark, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending', datetime('now'))",
        params![order_no, req.table_id, req.customer_name, req.customer_phone, req.estimated_amount, req.remark]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_pending_orders().into_iter().find(|p| p.id == id).ok_or("创建挂单失败".to_string())
}

pub fn cancel_pending_order(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("UPDATE pending_orders SET status = 'cancelled' WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn resume_pending_order(id: i64) -> Result<PendingOrder, String> {
    let conn = DB.lock();
    conn.execute("UPDATE pending_orders SET status = 'pending' WHERE id = ?1 AND status = 'suspended'", params![id]).map_err(|e| e.to_string())?;
    let rows_affected = conn.changes();
    if rows_affected == 0 { return Err("挂单不存在或已恢复".to_string()); }
    let order = conn.query_row("SELECT p.id, p.order_no, p.table_id, t.name, p.customer_name, p.customer_phone, p.estimated_amount, p.remark, p.status, p.created_at FROM pending_orders p LEFT JOIN tables t ON p.table_id = t.id WHERE p.id = ?1", params![id], |row| Ok(PendingOrder {
        id: row.get(0)?, order_no: row.get(1)?, table_id: row.get(2)?, table_name: row.get(3).unwrap_or_default(),
        customer_name: row.get(4)?, customer_phone: row.get(5)?, estimated_amount: row.get(6).unwrap_or(0.0),
        remark: row.get(7)?, status: row.get(8)?, created_at: row.get(9)?,
    })).map_err(|e| format!("恢复挂单失败: {}", e))?;
    drop(conn);
    Ok(order)
}
