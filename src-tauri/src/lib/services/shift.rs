use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::today_local;
use rusqlite::params;

pub fn get_current_shift() -> Option<ShiftRecord> {
    let conn = DB.lock();
    conn.query_row("SELECT s.id, s.shift_name, s.operator_id, u.username, s.start_time, s.end_time, s.revenue, s.order_count, s.status, COALESCE(s.handover_amount, 0.0) FROM shift_records s LEFT JOIN users u ON s.operator_id = u.id WHERE s.status = 'active' ORDER BY s.start_time DESC LIMIT 1", [], |row| {
        Ok(ShiftRecord { id: row.get(0)?, shift_name: row.get(1)?, operator_id: row.get(2)?, operator_name: row.get(3)?, start_time: row.get(4)?, end_time: row.get(5)?, revenue: row.get(6).unwrap_or(0.0), order_count: row.get(7).unwrap_or(0), status: row.get(8)?, handover_amount: row.get(9).unwrap_or(0.0) })
    }).ok()
}

pub fn get_shift_records() -> Vec<ShiftRecord> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT s.id, s.shift_name, s.operator_id, u.username, s.start_time, s.end_time, s.revenue, s.order_count, s.status, COALESCE(s.handover_amount, 0.0) FROM shift_records s LEFT JOIN users u ON s.operator_id = u.id ORDER BY s.start_time DESC") { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<ShiftRecord> = match stmt.query_map([], |row| {
        Ok(ShiftRecord { id: row.get(0)?, shift_name: row.get(1)?, operator_id: row.get(2)?, operator_name: row.get(3)?, start_time: row.get(4)?, end_time: row.get(5)?, revenue: row.get(6).unwrap_or(0.0), order_count: row.get(7).unwrap_or(0), status: row.get(8)?, handover_amount: row.get(9).unwrap_or(0.0) })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn get_shift_stats() -> ShiftStats {
    let conn = DB.lock();
    let today = today_local();
    // P2 #17 修复: 使用 SQLite date() 函数配合本地时区偏移，将 UTC 时间转换为本地日期
    // date(end_time, '+8 hours') 将 UTC 转为北京时间 (假设系统时区为 Asia/Shanghai)
    let completed_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM orders WHERE date(end_time, '+8 hours') = ?1 AND status = '已结账'", 
        params![today], |r| r.get(0)
    ).unwrap_or(0);
    let pending_count: i32 = conn.query_row("SELECT COUNT(*) FROM orders WHERE status = '进行中'", [], |r| r.get(0)).unwrap_or(0);
    let revenue: f64 = conn.query_row("SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time, '+8 hours') = ?1 AND status = '已结账'", params![today], |r| r.get(0)).unwrap_or(0.0);
    let cash_amount: f64 = conn.query_row("SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time, '+8 hours') = ?1 AND status = '已结账' AND payment_method = 'cash'", params![today], |r| r.get(0)).unwrap_or(0.0);
    let wechat_amount: f64 = conn.query_row("SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time, '+8 hours') = ?1 AND status = '已结账' AND payment_method = 'wechat'", params![today], |r| r.get(0)).unwrap_or(0.0);
    let alipay_amount: f64 = conn.query_row("SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time, '+8 hours') = ?1 AND status = '已结账' AND payment_method = 'alipay'", params![today], |r| r.get(0)).unwrap_or(0.0);
    let member_amount: f64 = conn.query_row("SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time, '+8 hours') = ?1 AND status = '已结账' AND member_id IS NOT NULL", params![today], |r| r.get(0)).unwrap_or(0.0);
    ShiftStats { revenue, order_count: completed_count + pending_count, completed_count, pending_count, cash_amount, wechat_amount, alipay_amount, member_amount }
}

pub fn create_shift_record(req: CreateShiftRecordRequest) -> Result<ShiftRecord, String> {
    let conn = DB.lock();
    
    // Close previous active shift and record its stats
    let prev_shift: Option<(i64, String)> = conn.query_row(
        "SELECT id, shift_name FROM shift_records WHERE status = 'active' ORDER BY start_time DESC LIMIT 1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?))
    ).ok();
    
    if let Some((prev_id, _)) = prev_shift {
        // Update previous shift with actual stats and handover amount
        let stats = get_shift_stats();
        conn.execute(
            "UPDATE shift_records SET status = 'completed', end_time = datetime('now'), revenue = ?1, order_count = ?2 WHERE id = ?3",
            params![stats.revenue, stats.order_count, prev_id],
        ).ok();
    }
    
    let operator_name: String = conn.query_row("SELECT username FROM users WHERE id = ?1", params![req.next_operator_id], |r| r.get(0)).map_err(|_| "用户不存在")?;
    let shift_name = format!("{} 的班次", operator_name);
    
    // Fix #6: Record handover_amount in the new shift
    conn.execute(
        "INSERT INTO shift_records (shift_name, operator_id, start_time, revenue, order_count, status, handover_amount) VALUES (?1, ?2, datetime('now'), 0, 0, 'active', ?3)",
        params![shift_name, req.next_operator_id, req.handover_amount],
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    Ok(ShiftRecord { id, shift_name, operator_id: req.next_operator_id, operator_name, start_time: chrono::Utc::now().to_rfc3339(), end_time: None, revenue: 0.0, order_count: 0, status: "active".to_string(), handover_amount: req.handover_amount })
}
